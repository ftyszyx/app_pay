export interface ApiResponse<T> {
      code: number;
      message: string;
      data: T;
}



export interface AuthPayload{
  username: string;
  password: string;
}

export interface AuthResponse{
  token: string;
}

export interface RegisterPayload{
  username: string;
  password: string;
}