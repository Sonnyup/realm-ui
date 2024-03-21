// 配置项

type Record = {
    local_host: string,
    local_port: number,
    remote_host: string,
    remote_port: number,
    protocol: string[],
    pid: number
}

export type { Record };