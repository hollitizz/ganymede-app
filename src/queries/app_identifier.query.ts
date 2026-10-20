import { queryOptions } from '@tanstack/react-query'
import { getIdentifier } from '@tauri-apps/api/app'

export const OFFICIAL_APP_IDENTIFIER = 'com.ganymede.ganymede-app'

export const appIdentifierQuery = queryOptions({
  queryKey: ['app', 'identifier'],
  queryFn: () => {
    return getIdentifier()
  },
})
