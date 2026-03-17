function apiBase(): string {
  if (typeof window === 'undefined') {
    return ''
  }

  const protocol = window.location.protocol
  return protocol === 'http:' || protocol === 'https:' ? '' : 'http://127.0.0.1:8787'
}

export async function pickWorkspaceRoot(): Promise<string | null> {
  if (typeof window === 'undefined') {
    return null
  }

  const protocol = window.location.protocol
  if (protocol === 'http:' || protocol === 'https:') {
    return null
  }

  const { open } = await import('@tauri-apps/plugin-dialog')
  const result = await open({
    directory: true,
    multiple: false,
  })

  if (Array.isArray(result)) {
    return typeof result[0] === 'string' ? result[0] : null
  }

  return typeof result === 'string' ? result : null
}

export async function fetchJson<T>(path: string): Promise<T> {
  const response = await fetch(`${apiBase()}${path}`)
  if (!response.ok) {
    throw new Error(await response.text())
  }

  return response.json() as Promise<T>
}

export async function postJson<TResponse, TBody>(path: string, body: TBody): Promise<TResponse> {
  const response = await fetch(`${apiBase()}${path}`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  })

  if (!response.ok) {
    throw new Error(await response.text())
  }

  return response.json() as Promise<TResponse>
}
