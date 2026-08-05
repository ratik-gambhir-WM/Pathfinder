export const TAURI_COMMANDS = {
  createUser: "create_user",
  createWmUser: "create_wm_user",
  getWmUserByEmail: "get_wm_user_by_email",
  databaseStatus: "database_status",
  extractDealQuestionsAndThesis: "extract_deal_questions_and_thesis",
  getUserByEmail: "get_user_by_email",
  greet: "greet",
  listDealDataRoom: "list_deal_data_room",
  listSummaryFiles: "list_summary_files",
  loginDemoCommand: "login_demo_command",
  previewDealDocument: "preview_deal_document",
  saveMarkdownSummary: "save_markdown_summary",
  saveDealAndExtract: "save_deal_and_extract",
  summarize: "summarize",
  summarizeSelected: "summarize_selected",
  userExistsByEmail: "user_exists_by_email",
} as const;

export type TauriCommandName = (typeof TAURI_COMMANDS)[keyof typeof TAURI_COMMANDS];
