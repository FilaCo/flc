import { browser } from '$app/environment'
import { writable } from 'svelte/store'

const persistedStorage = <T>(key: string, defaultValue?: T) => {
  if (!browser) {
    return writable(defaultValue)
  }

  const value = localStorage.getItem(key)
  const store = writable(value == null ? defaultValue : JSON.parse(value))
  store.subscribe((v) => localStorage.setItem(key, JSON.stringify(v)))

  return store
}

export default persistedStorage
