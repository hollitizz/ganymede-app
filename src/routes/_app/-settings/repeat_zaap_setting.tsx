import { Trans } from '@lingui/react/macro'
import { useSuspenseQuery } from '@tanstack/react-query'

import { Label } from '@/components/ui/label.tsx'
import { Switch } from '@/components/ui/switch.tsx'
import { useSetConf } from '@/mutations/set_conf.mutation.ts'
import { confQuery } from '@/queries/conf.query.ts'

export function RepeatZaapSetting() {
  const conf = useSuspenseQuery(confQuery)
  const setConf = useSetConf()

  return (
    <div className="flex flex-col gap-1">
      <div className="flex items-center justify-between gap-2">
        <Label className="text-xs" htmlFor="repeat-zaap">
          <Trans>Doubler /zaap avant un /travel</Trans>
        </Label>

        <Switch
          id="repeat-zaap"
          checked={conf.data.repeatZaapBeforeTravel ?? false}
          onCheckedChange={(checked) => setConf.mutate({ ...conf.data, repeatZaapBeforeTravel: checked })}
        />
      </div>
      <p className="text-xs text-muted-foreground">
        <Trans>Contournement d'un bug du jeu, à activer seulement s'il revient.</Trans>
      </p>
    </div>
  )
}
