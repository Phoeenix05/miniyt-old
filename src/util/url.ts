export const construct_url = (path: string, query?: string): string => {
    const BASE_URL = "https://inv.zzls.xyz/api/v1"
    const url = `${BASE_URL}/${path}/?${query != undefined ? query : ""}`
    return url
}
