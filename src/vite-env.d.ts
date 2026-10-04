interface ImportMetaEnv {
  readonly PUBLIC_IS_WEB?: string;
  readonly PUBLIC_API_URL?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
