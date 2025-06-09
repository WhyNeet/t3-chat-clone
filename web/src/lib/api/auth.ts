import { BACKEND_URI } from "../constants"
import type { User } from "../user";

export interface LoginPayload {
  email: string,
  password: string
}

export async function login(payload: LoginPayload): Promise<User> {
  const result = await fetch(`${BACKEND_URI}/auth/login`, {
    method: "POST",
    body: JSON.stringify(payload),
    headers: {
      "Content-Type": "application/json"
    },
    credentials: "include"
  });
  const user = await result.json();

  return user;
}

export async function me(): Promise<User> {
  const result = await fetch(`${BACKEND_URI}/users/me`, {
    method: "GET",
    headers: {
      Accept: "application/json"
    },
    credentials: "include"
  });
  const user = await result.json();

  return user;
}
