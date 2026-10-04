interface ImportMetaEnv {
  readonly IS_WEB?: string;
  readonly VITE_API_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
