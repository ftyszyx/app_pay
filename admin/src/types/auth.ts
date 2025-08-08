
export interface AuthPayload {
    username: string;
    password: string;
}

export interface AuthResponse {
    token: string;
}

export interface RegisterPayload {
    username: string;
    password: string;
}