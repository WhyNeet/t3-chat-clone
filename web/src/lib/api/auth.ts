import { BACKEND_URI } from "../constants"
import type { User } from "../model/user";

export interface LoginPayload {
  email: string,
  password: string
}

export async function login(payload: LoginPayload): Promise<User | { error: string }> {
  const result = await fetch(`${BACKEND_URI}/auth/login`, {
    method: "POST",
    body: JSON.stringify(payload),
    headers: {
      "Content-Type": "application/json"
    },
    credentials: "include"
  })
  const user = await result.json();

  return user;
}

export interface SignupPayload {
  email: string,
  password: string
}

export async function signup(payload: SignupPayload): Promise<User | { error: string }> {
  const result = await fetch(`${BACKEND_URI}/auth/register`, {
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

export async function me(): Promise<User | null> {
  const response = await fetch(`${BACKEND_URI}/users/me`, {
    method: "GET",
    headers: {
      Accept: "application/json"
    },
    credentials: "include"
  });
  if (response.status !== 200) return null;
  const user = await response.json();

  return user;
}
