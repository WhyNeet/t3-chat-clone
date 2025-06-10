export interface ApiError {
  error: string
}

export const isError = (response: unknown): response is ApiError => "error" in (response as Record<string, string>);
