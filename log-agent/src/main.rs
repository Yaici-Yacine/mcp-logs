mod capture;
mod cli;
mod config;
mod socket;
mod supervisor;
mod tui;
mod types;

use capture::ProcessCapture;
use clap::Parser;
use cli::{Cli, Commands, ConfigAction};
use config::Config;
use owo_colors::OwoColorize;
use socket::SocketClient;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { project, verbose, watch, cmd, command } => {
            run_command(project, verbose, watch, cmd, command).await?;
        }
        Commands::Test { message } => {
            test_connection(message).await?;
        }
        Commands::Config { action } => {
            handle_config_command(action)?;
        }
    }

    Ok(())
}

async fn run_command(
    project_override: Option<String>, 
    verbose_override: bool, 
    watch: bool, 
    cmd_name: Option<String>,
    command_args: Vec<String>
) -> Result<(), Box<dyn std::error::Error>> {
    // Charger la configuration
    let mut config = config::load_config().unwrap_or_else(|e| {
        eprintln!("{}", format!("Warning: Failed to load config: {}", e).yellow());
        eprintln!("  Using default configuration");
        Config::default()
    });
    
    // Déterminer la commande à exécuter et le mode watch
    // Priorité: CLI args > --cmd > default_command
    let (command, cmd_watch_override) = if !command_args.is_empty() {
        // 1. Commande fournie en CLI arguments (pas de watch override spécifique)
        (command_args, None)
    } else if let Some(name) = cmd_name {
        // 2. Commande prédéfinie via --cmd
        if let Some(cmd_config) = config.agent.commands.get(&name) {
            // Extraire la commande et le watch override depuis CommandConfig
            match cmd_config {
                config::CommandConfig::Simple(cmd) => (cmd.clone(), None),
                config::CommandConfig::Detailed { command: cmd, watch: cmd_watch } => {
                    (cmd.clone(), Some(*cmd_watch))
                }
            }
        } else {
            eprintln!("{}", format!("Error: Predefined command '{}' not found in config", name).red());
            eprintln!();
            eprintln!("Available commands in config:");
            if config.agent.commands.is_empty() {
                eprintln!("  (none defined)");
                eprintln!();
                eprintln!("To define commands, edit your config file:");
                eprintln!("  mcp-log-agent config init --local");
                eprintln!("  # Then add commands in [agent.commands] section:");
                eprintln!("  # [agent.commands]");
                eprintln!("  # dev = [\"npm\", \"run\", \"dev\"]");
                eprintln!("  # test = {{ command = [\"npm\", \"test\"], watch = true }}");
            } else {
                for (name, cmd_config) in &config.agent.commands {
                    match cmd_config {
                        config::CommandConfig::Simple(cmd) => {
                            eprintln!("  {} = {:?}", name.bright_cyan(), cmd);
                        }
                        config::CommandConfig::Detailed { command: cmd, watch: cmd_watch } => {
                            eprintln!("  {} = {:?} (watch: {})", name.bright_cyan(), cmd, cmd_watch);
                        }
                    }
                }
            }
            return Err(format!("Predefined command '{}' not found", name).into());
        }
    } else if let Some(default_cmd) = &config.agent.default_command {
        // 3. Commande par défaut de la config (pas de watch override spécifique)
        (default_cmd.clone(), None)
    } else {
        // Aucune commande spécifiée
        eprintln!("{}", "Error: No command provided".red());
        eprintln!();
        eprintln!("Usage:");
        eprintln!("  1. Provide a command:");
        eprintln!("     mcp-log-agent run -- npm start");
        eprintln!();
        eprintln!("  2. Use a predefined command:");
        eprintln!("     mcp-log-agent run --cmd dev");
        eprintln!();
        eprintln!("  3. Or set default_command in config:");
        eprintln!("     mcp-log-agent config init --local");
        eprintln!("     # Then edit .mcp-log-agent.toml:");
        eprintln!("     # default_command = [\"npm\", \"start\"]");
        eprintln!();
        eprintln!("  4. Then simply run:");
        eprintln!("     mcp-log-agent run");
        return Err("No command specified".into());
    };
    
    // Appliquer les overrides CLI
    if let Some(proj) = project_override {
        config.agent.default_project = proj;
    }
    if verbose_override {
        config.agent.verbose = true;
    }
    
    // Déterminer le mode watch (priorité: CLI flag > commande spécifique > config globale)
    let use_watch = watch || cmd_watch_override.unwrap_or(config.agent.watch);
    
    let project = config.agent.default_project.clone();
    
    // Mode TUI avec supervision (--watch ou config.agent.watch = true)
    if use_watch {
        return tui::run_tui(project, command, config)
            .await
            .map_err(|e| -> Box<dyn std::error::Error> { Box::new(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())) });
    }
    
    // Mode classique (one-shot)
    // Afficher les informations
    if config::has_local_config() {
        eprintln!("{}", "Using local configuration".bright_green());
        eprintln!("  Location: ./.mcp-log-agent.toml");
    } else if config::has_global_config() {
        eprintln!("{}", "Using global configuration".bright_cyan());
        if let Some(path) = config::get_global_config_path() {
            eprintln!("  Location: {}", path.display());
        }
    }
    
    eprintln!("{}", format!("Project: {}", project).bright_cyan());
    eprintln!();

    // Créer un channel pour les logs
    let (tx, rx) = mpsc::channel(config.performance.buffer_size);

    // Démarrer le worker socket dans une tâche séparée
    let socket_path = config.agent.socket_path.clone();
    let socket_client = SocketClient::new(Some(socket_path));
    let socket_task = tokio::spawn(async move {
        if let Err(e) = socket_client.start_worker(rx).await {
            eprintln!("{}", format!("Socket worker error: {}", e).red());
        }
    });

    // Créer et lancer la capture du processus
    let capture = ProcessCapture::new(project, command, config.clone());

    // Lancer la capture (bloquant jusqu'à ce que le processus se termine)
    let capture_handle = capture.spawn_with_tx(tx);

    // Attendre la fin du processus
    match capture_handle.await {
        Ok(Ok(_)) => {},
        Ok(Err(e)) => {
            eprintln!("{}", format!("\nProcess error: {}", e).red());
        }
        Err(e) => {
            eprintln!("{}", format!("\nTask error: {}", e).red());
        }
    }

    // Attendre que le worker socket se termine
    let _ = socket_task.await;

    Ok(())
}

async fn test_connection(message: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_config().unwrap_or_default();
    let socket_client = SocketClient::new(Some(config.agent.socket_path.clone()));
    socket_client.test_connection(message).await
}

/// Gère l'ajout du fichier de config local au .gitignore
fn handle_gitignore_for_local_config(no_gitignore: bool, auto_yes: bool) -> Result<(), Box<dyn std::error::Error>> {
    use owo_colors::OwoColorize;
    
    let config_filename = ".mcp-log-agent.toml";
    
    // Si --no-gitignore est spécifié, skip
    if no_gitignore {
        return Ok(());
    }
    
    // Vérifier si on est dans un repo Git
    if !config::is_git_repository() {
        return Ok(());
    }
    
    // Vérifier si le fichier est déjà dans .gitignore
    if config::is_config_in_gitignore(config_filename)? {
        println!("{}", "  ℹ Already in .gitignore".bright_black());
        return Ok(());
    }
    
    // Si --yes est spécifié, ajouter automatiquement
    if auto_yes {
        config::add_to_gitignore(config_filename)?;
        println!("{}", "✓ Added to .gitignore".green());
        return Ok(());
    }
    
    // Sinon, demander à l'utilisateur
    if config::prompt_add_to_gitignore(config_filename) {
        config::add_to_gitignore(config_filename)?;
        println!("{}", "✓ Added to .gitignore".green());
    } else {
        println!("{}", "  Skipped adding to .gitignore".bright_black());
    }
    
    Ok(())
}

fn handle_config_command(action: ConfigAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ConfigAction::Init { global, local, no_gitignore, yes } => {
            if global {
                if let Some(path) = config::get_global_config_path() {
                    config::create_default_config(&path)?;
                    println!("{}", "✓ Created global configuration file".green());
                    println!("  Location: {}", path.display());
                } else {
                    eprintln!("{}", "✗ Could not determine global config directory".red());
                }
            } else if local {
                let path = config::get_local_config_path();
                config::create_default_config(&path)?;
                println!("{}", "✓ Created local configuration file".green());
                println!("  Location: {}", path.display());
                
                // Vérifier si c'est un repo Git et proposer d'ajouter au .gitignore
                handle_gitignore_for_local_config(no_gitignore, yes)?;
            } else {
                // Par défaut, créer config locale
                let path = config::get_local_config_path();
                config::create_default_config(&path)?;
                println!("{}", "✓ Created local configuration file".green());
                println!("  Location: {}", path.display());
                
                // Vérifier si c'est un repo Git et proposer d'ajouter au .gitignore
                handle_gitignore_for_local_config(no_gitignore, yes)?;
            }
        }
        ConfigAction::Show { json } => {
            let config = config::load_config()?;
            if json {
                println!("{}", serde_json::to_string_pretty(&config)?);
            } else {
                println!("{}", toml::to_string_pretty(&config)?);
            }
        }
        ConfigAction::Get { key } => {
            let config = config::load_config()?;
            let toml_str = toml::to_string(&config)?;
            let value: toml::Value = toml::from_str(&toml_str)?;
            
            let parts: Vec<&str> = key.split('.').collect();
            let mut current = &value;
            
            for part in parts {
                if let Some(table) = current.as_table() {
                    if let Some(val) = table.get(part) {
                        current = val;
                    } else {
                        eprintln!("{}", format!("✗ Key not found: {}", key).red());
                        return Ok(());
                    }
                } else {
                    eprintln!("{}", format!("✗ Invalid key path: {}", key).red());
                    return Ok(());
                }
            }
            
            println!("{}", current);
        }
        ConfigAction::Set { global, key, value } => {
            let path = if global {
                config::get_global_config_path().ok_or("Could not determine global config path")?
            } else {
                config::get_local_config_path()
            };
            
            // Utiliser la nouvelle fonction set_config_value
            match config::set_config_value(&path, &key, &value) {
                Ok(_) => {
                    println!("{}", format!("✓ Configuration updated: {} = {}", key, value).green());
                    println!("  File: {}", path.display());
                }
                Err(e) => {
                    eprintln!("{}", format!("✗ Failed to set config: {}", e).red());
                    return Err(e);
                }
            }
        }
        ConfigAction::List => {
            println!("Available configuration keys:");
            println!();
            println!("{}", "[agent]".bright_cyan());
            println!("  socket_path           - Socket path for MCP server");
            println!("  default_project       - Default project name");
            println!("  verbose               - Verbose output");
            println!("  connection_timeout    - Connection timeout in seconds");
            println!("  retry_attempts        - Number of retry attempts");
            println!();
            println!("{}", "[output]".bright_cyan());
            println!("  colors                - Enable colored output");
            println!("  format                - Output format (colored, plain, json)");
            println!("  show_timestamps       - Show timestamps in output");
            println!("  show_pid              - Show process ID in output");
            println!();
            println!("{}", "[colors]".bright_cyan());
            println!("  See 'mcp-log-agent config colors list' for color configuration");
        }
        ConfigAction::Validate => {
            println!("Validating configuration...");
            
            let mut has_errors = false;
            
            if let Some(global_path) = config::get_global_config_path() {
                if global_path.exists() {
                    match config::load_config() {
                        Ok(_) => println!("{}", "  Global config: ✓ Valid".green()),
                        Err(e) => {
                            println!("{}", format!("  Global config: ✗ Error: {}", e).red());
                            has_errors = true;
                        }
                    }
                }
            }
            
            let local_path = config::get_local_config_path();
            if local_path.exists() {
                match config::load_config() {
                    Ok(_) => println!("{}", "  Local config:  ✓ Valid".green()),
                    Err(e) => {
                        println!("{}", format!("  Local config:  ✗ Error: {}", e).red());
                        has_errors = true;
                    }
                }
            }
            
            if !has_errors {
                println!();
                println!("{}", "✓ Configuration is valid".green());
            }
        }
        ConfigAction::Detect => {
            println!("Configuration Detection");
            println!();
            
            if let Some(global_path) = config::get_global_config_path() {
                if global_path.exists() {
                    println!("{}", "  ✓ Global config".green());
                    println!("      {}", global_path.display());
                } else {
                    println!("{}", "  ✗ Global config not found".yellow());
                }
            }
            
            let local_path = config::get_local_config_path();
            if local_path.exists() {
                println!("{}", "  ✓ Local config".green());
                println!("      {}", local_path.display());
            } else {
                println!("{}", "  ✗ Local config not found".yellow());
            }
            
            println!();
            let config = config::load_config().unwrap_or_default();
            println!("Default project when running from this directory:");
            println!("  → {}", config.agent.default_project.bright_cyan());
        }
        ConfigAction::Reset { global, local } => {
            if global {
                if let Some(path) = config::get_global_config_path() {
                    config::create_default_config(&path)?;
                    println!("{}", "✓ Reset global configuration to defaults".green());
                }
            } else if local {
                let path = config::get_local_config_path();
                config::create_default_config(&path)?;
                println!("{}", "✓ Reset local configuration to defaults".green());
            } else {
                println!("{}", "Please specify --global or --local".yellow());
            }
        }
        ConfigAction::Theme { action } => {
            handle_theme_action(action)?;
        }
    }
    
    Ok(())
}

fn handle_theme_action(action: cli::ThemeAction) -> Result<(), Box<dyn std::error::Error>> {
    use cli::ThemeAction;
    use config::themes::ThemeManager;
    
    // Créer le gestionnaire de thèmes
    let config_dir = config::get_global_config_dir()
        .ok_or("Could not determine global config directory")?;
    let theme_manager = ThemeManager::new(config_dir);
    
    // Initialiser les thèmes par défaut si nécessaire
    theme_manager.initialize_default_themes()?;
    
    match action {
        ThemeAction::List => {
            println!("{}", "Available themes:".bright_cyan().bold());
            println!();
            
            match theme_manager.list_themes_with_info() {
                Ok(themes) => {
                    if themes.is_empty() {
                        println!("{}", "  No themes found".yellow());
                        println!("  Run 'mcp-log-agent config theme create <name>' to create one");
                    } else {
                        for (name, description, author) in themes {
                            print!("  {} ", name.bright_cyan().bold());
                            if let Some(desc) = description {
                                print!("- {}", desc);
                            }
                            if let Some(auth) = author {
                                print!(" {}", format!("(by {})", auth).bright_black());
                            }
                            println!();
                        }
                    }
                }
                Err(e) => {
                    eprintln!("{}", format!("✗ Failed to list themes: {}", e).red());
                }
            }
            
            println!();
            println!("To apply a theme:");
            println!("  {}", "mcp-log-agent config theme set <name>".bright_black());
        }
        
        ThemeAction::Show { name } => {
            match theme_manager.load_theme(&name) {
                Ok(theme) => {
                    println!("{}", format!("Theme: {}", theme.name).bright_cyan().bold());
                    if let Some(desc) = &theme.description {
                        println!("Description: {}", desc);
                    }
                    if let Some(author) = &theme.author {
                        println!("Author: {}", author);
                    }
                    println!();
                    
                    // Afficher le contenu du thème en TOML
                    match toml::to_string_pretty(&theme) {
                        Ok(content) => println!("{}", content),
                        Err(e) => eprintln!("{}", format!("✗ Failed to serialize theme: {}", e).red()),
                    }
                }
                Err(e) => {
                    eprintln!("{}", format!("✗ Theme '{}' not found: {}", name, e).red());
                    eprintln!("  Use 'mcp-log-agent config theme list' to see available themes");
                }
            }
        }
        
        ThemeAction::Create { name, from, interactive } => {
            // Vérifier si le thème existe déjà
            if theme_manager.theme_exists(&name) {
                eprintln!("{}", format!("✗ Theme '{}' already exists", name).red());
                eprintln!("  Choose a different name or delete the existing theme first");
                return Ok(());
            }
            
            let theme = if let Some(template) = from {
                // Créer depuis un template
                match theme_manager.create_from_template(&name, &template) {
                    Ok(theme) => theme,
                    Err(e) => {
                        eprintln!("{}", format!("✗ Failed to create from template: {}", e).red());
                        return Ok(());
                    }
                }
            } else if interactive {
                // Mode interactif - pour l'instant, on crée un template basique
                // TODO: Implémenter le mode interactif avec dialoguer
                eprintln!("{}", "✗ Interactive mode not yet implemented".red());
                eprintln!("  Use --from <template> to create from an existing theme");
                eprintln!("  Example: mcp-log-agent config theme create my-theme --from default");
                return Ok(());
            } else {
                // Créer depuis le thème par défaut
                match theme_manager.create_from_template(&name, "default") {
                    Ok(theme) => theme,
                    Err(e) => {
                        eprintln!("{}", format!("✗ Failed to create theme: {}", e).red());
                        return Ok(());
                    }
                }
            };
            
            // Sauvegarder le thème
            match theme_manager.save_theme(&theme) {
                Ok(_) => {
                    println!("{}", format!("✓ Created theme '{}'", name).green());
                    println!("  Location: {}/.config/mcp-log-agent/themes/{}.toml", 
                        std::env::var("HOME").unwrap_or_default(), name);
                    println!();
                    println!("  Edit the theme file to customize colors");
                    println!("  Apply with: {}", format!("mcp-log-agent config theme set {}", name).bright_black());
                }
                Err(e) => {
                    eprintln!("{}", format!("✗ Failed to save theme: {}", e).red());
                }
            }
        }
        
        ThemeAction::Export { name, description, author } => {
            // Charger la config actuelle
            let config = config::load_config()?;
            
            // Créer le thème depuis la config
            let theme = theme_manager.export_from_config(
                &name,
                &config.colors,
                &config.performance.tui.colors,
                description,
                author,
            );
            
            // Sauvegarder
            match theme_manager.save_theme(&theme) {
                Ok(_) => {
                    println!("{}", format!("✓ Exported current colors as theme '{}'", name).green());
                    println!("  Location: {}/.config/mcp-log-agent/themes/{}.toml", 
                        std::env::var("HOME").unwrap_or_default(), name);
                }
                Err(e) => {
                    eprintln!("{}", format!("✗ Failed to export theme: {}", e).red());
                }
            }
        }
        
        ThemeAction::Set { name, global } => {
            // Vérifier que le thème existe
            if !theme_manager.theme_exists(&name) {
                eprintln!("{}", format!("✗ Theme '{}' not found", name).red());
                eprintln!("  Use 'mcp-log-agent config theme list' to see available themes");
                return Ok(());
            }
            
            // Déterminer quel fichier de config modifier
            let config_path = if global {
                config::get_global_config_path().ok_or("Could not determine global config path")?
            } else {
                if config::has_local_config() {
                    config::get_local_config_path()
                } else {
                    config::get_global_config_path().ok_or("Could not determine config path")?
                }
            };
            
            // Charger la config existante
            let mut config = if config_path.exists() {
                match config::load_config_from_file(&config_path) {
                    Ok(c) => c,
                    Err(_) => config::Config::default(),
                }
            } else {
                config::Config::default()
            };
            
            // Mettre à jour le champ theme
            config.theme = name.clone();
            
            // Sauvegarder
            match config::save_config(&config, &config_path) {
                Ok(_) => {
                    println!("{}", format!("✓ Set theme to '{}'", name).green());
                    println!("  File: {}", config_path.display());
                }
                Err(e) => {
                    eprintln!("{}", format!("✗ Failed to save config: {}", e).red());
                }
            }
        }
        
        ThemeAction::Preview { name } => {
            match theme_manager.load_theme(&name) {
                Ok(theme) => {
                    println!("{}", format!("Theme Preview: {}", theme.name).bright_cyan().bold());
                    if let Some(desc) = &theme.description {
                        println!("{}", desc.bright_black());
                    }
                    println!();
                    
                    println!("{}", "Log Levels:".bold());
                    println!("  {} This is an error message", "ERROR".red().bold());
                    println!("  {} This is a warning message", "WARN ".yellow());
                    println!("  {} This is an info message", "INFO ".cyan());
                    println!("  {} This is a debug message", "DEBUG".blue());
                    println!();
                    println!("{}", "System Messages:".bold());
                    println!("  {} Success: Operation completed", "✓".green().bold());
                    println!("  {} Error: Operation failed", "✗".red().bold());
                    println!("  {} Info: Additional information", "ℹ".cyan());
                    println!("    {}", "Dimmed text for secondary info".bright_black());
                }
                Err(e) => {
                    eprintln!("{}", format!("✗ Theme '{}' not found: {}", name, e).red());
                }
            }
        }
    }
    
    Ok(())
}
