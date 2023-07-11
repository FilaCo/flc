import type { Theme } from '$lib/utils/theming/theme'

const createCssVariables = (t: Theme): string => {
  let cssVariables = `--theme-name:${t.name}`

  for (const [colorProp, color] of Object.entries(t.colors)) {
    for (const [subColorProp, subColor] of Object.entries(color)) {
      cssVariables += `;--theme-${colorProp}-${subColorProp}:${subColor}`
    }
  }

  return cssVariables
}

export default createCssVariables
