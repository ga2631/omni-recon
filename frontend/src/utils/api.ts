/**
 * Resilient API Fetch Helper for OmniRecon
 * Automatically retries with exponential backoff if the backend is starting up (HTTP 502/503/504)
 */
export async function apiFetch(
  input: RequestInfo | URL,
  init?: RequestInit,
  maxRetries = 3,
  baseDelayMs = 800
): Promise<Response> {
  let lastError: any = null

  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      const res = await fetch(input, init)
      // If response is OK, or a valid business error (e.g. 400, 401, 404, 422), return immediately
      if (res.status !== 502 && res.status !== 503 && res.status !== 504) {
        return res
      }
    } catch (err) {
      lastError = err
    }

    if (attempt < maxRetries - 1) {
      await new Promise((resolve) => setTimeout(resolve, baseDelayMs * (attempt + 1)))
    }
  }

  if (lastError) {
    throw lastError
  }

  return fetch(input, init)
}
