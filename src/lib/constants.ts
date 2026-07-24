export const TAURI_COMMANDS = {
  createUser: "create_user",
  createWmUser: "create_wm_user",
  databaseStatus: "database_status",
  getUserByEmail: "get_user_by_email",
  greet: "greet",
  listSummaryFiles: "list_summary_files",
  loginDemoCommand: "login_demo_command",
  saveMarkdownSummary: "save_markdown_summary",
  summarize: "summarize",
  summarizeSelected: "summarize_selected",
  userExistsByEmail: "user_exists_by_email",
} as const;

export type TauriCommandName = (typeof TAURI_COMMANDS)[keyof typeof TAURI_COMMANDS];
