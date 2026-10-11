export interface BunRequest<P = any> {
    cmd: string;
    payload: P;
}
