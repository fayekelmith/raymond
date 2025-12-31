import { cookies } from "next/headers";

const SESSION_COOKIE_NAME = "raymond_session";

export function verifyCredentials(username: string, password: string): boolean {
  const adminUsername = process.env.ADMIN_USERNAME;
  const adminPassword = process.env.ADMIN_PASSWORD;

  if (!adminUsername || !adminPassword) {
    console.error("Admin credentials not set in environment");
    return false;
  }

  return username === adminUsername && password === adminPassword;
}

export async function createSession(): Promise<void> {
  const cookieStore = await cookies();
  // Simple session token - in production you'd want something more secure
  const sessionToken = Buffer.from(`${Date.now()}-${Math.random()}`).toString(
    "base64"
  );

  cookieStore.set(SESSION_COOKIE_NAME, sessionToken, {
    httpOnly: true,
    secure: process.env.NODE_ENV === "production",
    sameSite: "lax",
    maxAge: 60 * 60 * 24 * 7, // 1 week
    path: "/",
  });
}

export async function isAuthenticated(): Promise<boolean> {
  const cookieStore = await cookies();
  const session = cookieStore.get(SESSION_COOKIE_NAME);
  return !!session?.value;
}

export async function logout(): Promise<void> {
  const cookieStore = await cookies();
  cookieStore.delete(SESSION_COOKIE_NAME);
}
