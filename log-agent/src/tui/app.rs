use crate::config::Config;
use crate::types::{LogLevel, LogMessage, LogSource};
use regex::{Regex, RegexBuilder};
use std::collections::VecDeque;
use std::time::Instant;

/// État de l'application TUI
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Running,
    WaitingCountdown(u8),
    Restarting,
}

/// Mode d'interaction
#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Search,
    Help,
    SavePrompt,
}

/// Filtre par niveau de log
#[derive(Debug, Clone, PartialEq)]
pub enum LevelFilter {
    All,
    Error,
    Warn,
    Info,
    Debug,
}

impl LevelFilter {
    /// Passe au filtre suivant dans le cycle
    pub fn next(&self) -> Self {
        match self {
            LevelFilter::All => LevelFilter::Error,
            LevelFilter::Error => LevelFilter::Warn,
            LevelFilter::Warn => LevelFilter::Info,
            LevelFilter::Info => LevelFilter::Debug,
            LevelFilter::Debug => LevelFilter::All,
        }
    }

    /// Retourne le label pour l'affichage
    pub fn label(&self) -> &str {
        match self {
            LevelFilter::All => "ALL",
            LevelFilter::Error => "ERROR",
            LevelFilter::Warn => "WARN",
            LevelFilter::Info => "INFO",
            LevelFilter::Debug => "DEBUG",
        }
    }

    /// Vérifie si un niveau de log passe le filtre
    pub fn matches(&self, level: &LogLevel) -> bool {
        match self {
            LevelFilter::All => true,
            LevelFilter::Error => matches!(level, LogLevel::Error),
            LevelFilter::Warn => matches!(level, LogLevel::Warn),
            LevelFilter::Info => matches!(level, LogLevel::Info),
            LevelFilter::Debug => matches!(level, LogLevel::Debug),
        }
    }
}

/// Ligne de log pour l'affichage
#[derive(Debug, Clone)]
pub struct LogLine {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    #[allow(dead_code)]
    pub source: LogSource,
    pub is_system: bool,
}

impl From<LogMessage> for LogLine {
    fn from(log: LogMessage) -> Self {
        Self {
            timestamp: log.data.timestamp[11..19].to_string(), // HH:MM:SS
            level: log.data.level,
            message: log.data.message,
            source: log.data.source,
            is_system: false,
        }
    }
}

impl LogLine {
    pub fn system(message: String) -> Self {
        Self {
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            level: LogLevel::Info,
            message,
            source: LogSource::Stdout,
            is_system: true,
        }
    }
}

/// État principal de l'application
pub struct App {
    /// Buffer circulaire des logs
    pub logs: VecDeque<LogLine>,
    /// Limite de logs en mémoire
    pub max_logs: usize,
    /// Offset de scroll (0 = en bas, auto-scroll)
    pub scroll_offset: usize,
    /// Auto-scroll activé (suit les nouveaux logs)
    pub auto_scroll: bool,
    /// Ligne sélectionnée (pour copie, etc.)
    pub selected_line: Option<usize>,
    /// État du superviseur
    pub state: AppState,
    /// PID du processus enfant
    pub pid: Option<u32>,
    /// Heure de démarrage
    pub start_time: Instant,
    /// Flag pour quitter
    pub should_quit: bool,
    /// Nom du projet
    pub project: String,
    /// Commande exécutée
    pub command: Vec<String>,
    /// Configuration
    #[allow(dead_code)]
    pub config: Config,
    /// Hauteur visible des logs (mis à jour par ui.rs)
    pub visible_height: usize,
    /// Flag pour forcer le redraw
    pub needs_redraw: bool,

    // === Nouvelles fonctionnalités ===
    /// Mode d'input actuel
    pub input_mode: InputMode,
    /// Buffer de recherche / saisie
    pub input_buffer: String,
    pub search_regex: Option<Regex>,
    pub search_message: Option<String>,
    pub level_filter: LevelFilter,
    /// Pause de capture (n'ajoute pas de nouveaux logs)
    pub paused: bool,
    /// Buffer de logs en attente si pause
    pub paused_logs: Vec<LogMessage>,
    /// Stats réseau
    pub total_logs_received: usize,
    pub total_logs_sent: usize,
    pub last_log_time: Option<Instant>,

    // === Scroll horizontal ===
    /// Offset de scroll horizontal (nombre de caractères à sauter à gauche)
    pub horizontal_scroll: usize,

    // === Bookmarks ===
    /// Set des indices de lignes bookmarkées
    pub bookmarks: std::collections::HashSet<usize>,

    // === Help scroll ===
    /// Offset de scroll pour le popup d'aide
    pub help_scroll: usize,
}

impl App {
    pub fn new(project: String, command: Vec<String>, config: Config) -> Self {
        let max_logs = config.performance.tui.max_logs;

        Self {
            logs: VecDeque::with_capacity(max_logs),
            max_logs,
            scroll_offset: 0,
            auto_scroll: true,
            selected_line: None,
            state: AppState::Running,
            pid: None,
            start_time: Instant::now(),
            should_quit: false,
            project,
            command,
            config,
            visible_height: 20,
            needs_redraw: true,

            // Nouvelles fonctionnalités
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            search_regex: None,
            search_message: None,
            level_filter: LevelFilter::All,
            paused: false,
            paused_logs: Vec::new(),
            total_logs_received: 0,
            total_logs_sent: 0,
            last_log_time: None,
            horizontal_scroll: 0,
            bookmarks: std::collections::HashSet::new(),
            help_scroll: 0,
        }
    }

    /// Ajoute un log au buffer
    pub fn add_log(&mut self, log: LogMessage) {
        self.total_logs_received += 1;
        self.last_log_time = Some(Instant::now());

        // Si pause, stocker dans le buffer
        if self.paused {
            self.paused_logs.push(log);
            return;
        }

        self.logs.push_back(log.into());

        // Éviction FIFO si trop de logs
        while self.logs.len() > self.max_logs {
            self.logs.pop_front();
        }

        // Si auto-scroll, rester en bas
        if self.auto_scroll {
            self.scroll_offset = 0;
        }

        self.needs_redraw = true;
    }

    /// Ajoute un message système
    pub fn add_system_log(&mut self, message: String) {
        self.logs.push_back(LogLine::system(message));

        while self.logs.len() > self.max_logs {
            self.logs.pop_front();
        }

        if self.auto_scroll {
            self.scroll_offset = 0;
        }

        self.needs_redraw = true;
    }

    /// Clear tous les logs
    pub fn clear_logs(&mut self) {
        self.logs.clear();
        self.scroll_offset = 0;
        self.selected_line = None;
        self.add_system_log("Logs cleared".to_string());
        self.needs_redraw = true;
    }

    /// Scroll vers le haut
    pub fn scroll_up(&mut self, n: usize) {
        let max_offset = self.logs.len().saturating_sub(self.visible_height);
        self.scroll_offset = (self.scroll_offset + n).min(max_offset);
        self.auto_scroll = false;
        self.needs_redraw = true;
    }

    /// Scroll vers le bas
    pub fn scroll_down(&mut self, n: usize) {
        if self.scroll_offset >= n {
            self.scroll_offset -= n;
        } else {
            self.scroll_offset = 0;
            self.auto_scroll = true;
        }
        self.needs_redraw = true;
    }

    /// Scroll tout en haut
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = self.logs.len().saturating_sub(self.visible_height);
        self.auto_scroll = false;
        self.needs_redraw = true;
    }

    /// Scroll tout en bas (active auto-scroll)
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
        self.auto_scroll = true;
        self.selected_line = None;
        self.needs_redraw = true;
    }

    /// Sélectionne une ligne à la position Y donnée
    pub fn select_line_at(&mut self, row: usize) {
        // row est relatif au terminal, on doit calculer l'index du log
        // La zone des logs commence à row 1 (après le header)
        if row < 1 {
            return;
        }

        let log_row = row - 1;
        let total_logs = self.logs.len();

        if total_logs == 0 {
            return;
        }

        // Calculer l'index du log basé sur le scroll
        // Les logs sont affichés du plus ancien au plus récent
        // scroll_offset = 0 signifie qu'on voit les derniers logs
        let visible_start = total_logs.saturating_sub(self.visible_height + self.scroll_offset);
        let log_index = visible_start + log_row;

        if log_index < total_logs {
            self.selected_line = Some(log_index);
            self.auto_scroll = false;
            self.needs_redraw = true;
        }
    }

    /// Retourne le temps écoulé formaté
    pub fn uptime(&self) -> String {
        let elapsed = self.start_time.elapsed();
        let secs = elapsed.as_secs();

        if secs < 60 {
            format!("{}s", secs)
        } else if secs < 3600 {
            format!("{}m{}s", secs / 60, secs % 60)
        } else {
            format!("{}h{}m", secs / 3600, (secs % 3600) / 60)
        }
    }

    /// Set le PID
    pub fn set_pid(&mut self, pid: Option<u32>) {
        self.pid = pid;
        self.needs_redraw = true;
    }

    /// Set l'état
    pub fn set_state(&mut self, state: AppState) {
        self.state = state;
        self.needs_redraw = true;
    }

    /// Reset le timer de démarrage
    pub fn reset_start_time(&mut self) {
        self.start_time = Instant::now();
        self.needs_redraw = true;
    }

    /// Retourne la commande formatée
    pub fn command_str(&self) -> String {
        self.command.join(" ")
    }

    // === Nouvelles fonctionnalités ===

    /// Active le mode recherche
    pub fn enter_search_mode(&mut self) {
        self.input_mode = InputMode::Search;
        self.input_buffer.clear();
        self.search_message = None;
        self.needs_redraw = true;
    }

    /// Active le mode save
    pub fn enter_save_mode(&mut self) {
        self.input_mode = InputMode::SavePrompt;
        self.input_buffer = format!("{}_logs.txt", self.project);
        self.needs_redraw = true;
    }

    /// Active/désactive le mode aide
    pub fn toggle_help(&mut self) {
        self.input_mode = if self.input_mode == InputMode::Help {
            InputMode::Normal
        } else {
            InputMode::Help
        };
        self.help_scroll = 0; // Reset scroll when opening help
        self.needs_redraw = true;
    }

    /// Scroll help up
    pub fn scroll_help_up(&mut self) {
        self.help_scroll = self.help_scroll.saturating_sub(1);
        self.needs_redraw = true;
    }

    /// Scroll help down
    pub fn scroll_help_down(&mut self, max_scroll: usize) {
        if self.help_scroll < max_scroll {
            self.help_scroll += 1;
            self.needs_redraw = true;
        }
    }

    /// Quitte le mode input actuel
    pub fn exit_input_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();
        self.search_regex = None;
        self.search_message = None;
        self.needs_redraw = true;
    }

    /// Ajoute un caractère au buffer d'input
    pub fn input_char(&mut self, c: char) {
        self.input_buffer.push(c);
        self.needs_redraw = true;
    }

    /// Supprime le dernier caractère du buffer
    pub fn input_backspace(&mut self) {
        self.input_buffer.pop();
        self.needs_redraw = true;
    }

    /// Valide la recherche
    pub fn confirm_search(&mut self) {
        if self.input_buffer.is_empty() {
            // Clear search
            self.search_regex = None;
            self.search_message = None;
            self.input_mode = InputMode::Normal;
            self.needs_redraw = true;
            return;
        }

        match RegexBuilder::new(&self.input_buffer)
            .case_insensitive(true)
            .build()
        {
            Ok(regex) => {
                let matches = self
                    .logs
                    .iter()
                    .filter(|l| regex.is_match(&l.message))
                    .count();
                self.search_regex = Some(regex);
                self.search_message = Some(format!("{} matches", matches));
                self.input_mode = InputMode::Normal;
            }
            Err(e) => {
                self.search_message = Some(format!("Invalid regex: {}", e));
            }
        }
        self.needs_redraw = true;
    }

    /// Sauvegarde les logs dans un fichier
    pub fn save_logs(&mut self) -> Result<(), std::io::Error> {
        use std::io::Write;

        let filename = if self.input_buffer.is_empty() {
            format!("{}_logs.txt", self.project)
        } else {
            self.input_buffer.clone()
        };

        let mut file = std::fs::File::create(&filename)?;

        for log in &self.logs {
            writeln!(file, "[{}] {:?} {}", log.timestamp, log.level, log.message)?;
        }

        self.add_system_log(format!("Saved {} logs to {}", self.logs.len(), filename));
        self.input_mode = InputMode::Normal;
        self.input_buffer.clear();

        Ok(())
    }

    /// Toggle pause
    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;

        if !self.paused && !self.paused_logs.is_empty() {
            // Reprendre: ajouter les logs en attente
            let paused = std::mem::take(&mut self.paused_logs);
            for log in paused {
                self.logs.push_back(log.into());
            }

            // Éviction
            while self.logs.len() > self.max_logs {
                self.logs.pop_front();
            }

            self.add_system_log("Resumed capture".to_string());
        } else if self.paused {
            self.add_system_log("Paused capture".to_string());
        }

        self.needs_redraw = true;
    }

    /// Copie la ligne sélectionnée dans le presse-papier
    pub fn copy_selected_line(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(index) = self.selected_line {
            if let Some(log) = self.logs.get(index) {
                let text = format!("[{}] {:?} {}", log.timestamp, log.level, log.message);

                let mut clipboard = arboard::Clipboard::new()?;
                clipboard.set_text(text)?;

                self.add_system_log("Copied to clipboard".to_string());
            }
        } else {
            self.add_system_log("No line selected".to_string());
        }
        Ok(())
    }

    /// Cycle entre les filtres de niveau
    pub fn cycle_level_filter(&mut self) {
        self.level_filter = self.level_filter.next();
        self.needs_redraw = true;
    }

    /// Retourne les logs visibles filtrés par recherche ET par niveau
    pub fn filtered_visible_logs(&self) -> Vec<(usize, &LogLine, bool)> {
        let total = self.logs.len();
        let start = total.saturating_sub(self.visible_height + self.scroll_offset);
        let end = total.saturating_sub(self.scroll_offset);

        self.logs
            .iter()
            .enumerate()
            .skip(start)
            .take(end - start)
            .map(|(idx, log)| {
                // Un log matche si il passe TOUS les filtres
                let search_match = self
                    .search_regex
                    .as_ref()
                    .map(|re| re.is_match(&log.message))
                    .unwrap_or(true);

                let level_match = log.is_system || self.level_filter.matches(&log.level);

                let matches = search_match && level_match;
                (idx, log, matches)
            })
            .collect()
    }

    /// Retourne le nombre de logs filtrés
    pub fn filtered_count(&self) -> usize {
        self.logs
            .iter()
            .filter(|l| {
                let search_match = self
                    .search_regex
                    .as_ref()
                    .map(|re| re.is_match(&l.message))
                    .unwrap_or(true);

                let level_match = l.is_system || self.level_filter.matches(&l.level);

                search_match && level_match
            })
            .count()
    }

    /// Retourne les logs/sec
    pub fn logs_per_second(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.total_logs_received as f64 / elapsed
        } else {
            0.0
        }
    }

    /// Incrémente le compteur d'envoi
    pub fn increment_sent(&mut self) {
        self.total_logs_sent += 1;
    }

    // === Scroll horizontal ===

    /// Scroll vers la droite (afficher plus de caractères à droite)
    pub fn scroll_right(&mut self, n: usize) {
        self.horizontal_scroll += n;
        self.needs_redraw = true;
    }

    /// Scroll vers la gauche (retour au début de la ligne)
    pub fn scroll_left(&mut self, n: usize) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_sub(n);
        self.needs_redraw = true;
    }

    /// Reset le scroll horizontal
    pub fn reset_horizontal_scroll(&mut self) {
        self.horizontal_scroll = 0;
        self.needs_redraw = true;
    }

    // === Bookmarks ===

    /// Toggle bookmark sur la ligne sélectionnée
    pub fn toggle_bookmark(&mut self) {
        if let Some(idx) = self.selected_line {
            if self.bookmarks.contains(&idx) {
                self.bookmarks.remove(&idx);
                self.add_system_log("Bookmark removed".to_string());
            } else {
                self.bookmarks.insert(idx);
                self.add_system_log("Bookmark added".to_string());
            }
            self.needs_redraw = true;
        } else {
            self.add_system_log("No line selected. Click on a line first.".to_string());
        }
    }

    /// Va au bookmark suivant
    pub fn next_bookmark(&mut self) {
        if self.bookmarks.is_empty() {
            self.add_system_log("No bookmarks".to_string());
            return;
        }

        let current = self.selected_line.unwrap_or(0);
        let mut bookmarks: Vec<_> = self.bookmarks.iter().copied().collect();
        bookmarks.sort();

        // Trouver le prochain bookmark après la position actuelle
        if let Some(&next) = bookmarks.iter().find(|&&idx| idx > current) {
            self.selected_line = Some(next);
            // Scroller pour rendre visible
            self.scroll_to_line(next);
        } else if let Some(&first) = bookmarks.first() {
            // Wrap around au premier bookmark
            self.selected_line = Some(first);
            self.scroll_to_line(first);
        }

        self.needs_redraw = true;
    }

    /// Va au bookmark précédent
    pub fn prev_bookmark(&mut self) {
        if self.bookmarks.is_empty() {
            self.add_system_log("No bookmarks".to_string());
            return;
        }

        let current = self.selected_line.unwrap_or(self.logs.len());
        let mut bookmarks: Vec<_> = self.bookmarks.iter().copied().collect();
        bookmarks.sort();

        // Trouver le bookmark précédent avant la position actuelle
        if let Some(&prev) = bookmarks.iter().rev().find(|&&idx| idx < current) {
            self.selected_line = Some(prev);
            self.scroll_to_line(prev);
        } else if let Some(&last) = bookmarks.last() {
            // Wrap around au dernier bookmark
            self.selected_line = Some(last);
            self.scroll_to_line(last);
        }

        self.needs_redraw = true;
    }

    /// Clear tous les bookmarks
    pub fn clear_bookmarks(&mut self) {
        let count = self.bookmarks.len();
        self.bookmarks.clear();
        self.add_system_log(format!("Cleared {} bookmarks", count));
        self.needs_redraw = true;
    }

    /// Scroll pour rendre une ligne visible
    fn scroll_to_line(&mut self, line_idx: usize) {
        let total = self.logs.len();
        if line_idx >= total {
            return;
        }

        // Calculer où devrait être le scroll pour voir cette ligne
        // On veut que la ligne soit au milieu de l'écran si possible
        let target_offset = if line_idx + self.visible_height / 2 < total {
            total - line_idx - self.visible_height / 2
        } else {
            0
        };

        self.scroll_offset = target_offset;
        self.auto_scroll = false;
    }
}
