
/**
 * Used for log printing in a non-production environment
 * @param args
 */
export function consoleLog (...args: Parameters<typeof console.log>) {
  if (import.meta.env.DEV) {
    window.console.log.apply(null, args) // eslint-disable-line no-console
  }
}

export function consoleLogWithStack(...args: Parameters<typeof console.log>) {
  if (import.meta.env.DEV) {
    const stack = new Error().stack
    const stackStr = stack?.split('\n')[2].trim().trim()
    if (stackStr) {
      args.unshift(`[${stackStr}]`)
    }
    window.console.log.apply(null, args) // eslint-disable-line no-console
  }
}

export function consoleWarn (...args: Parameters<typeof console.warn>) {
  if (import.meta.env.DEV) {
    console.warn.apply(null, args) // eslint-disable-line no-console
  }
}

export function consoleError (...args: Parameters<typeof console.error>) {
  if (import.meta.env.DEV) {
    console.error.apply(null, args) // eslint-disable-line no-console
    console.trace()
  }
}

export function consoleErrorNoStack (...args: Parameters<typeof console.error>) {
  if (import.meta.env.DEV) {
    console.error.apply(null, args) // eslint-disable-line no-console
  }
}

export function testEnvLog (...args: Parameters<typeof console.log>) {
  if (import.meta.env.DEV) {
    console.log.apply(null, args) // eslint-disable-line no-console
  }
}
