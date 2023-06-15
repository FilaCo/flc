import type { Meta, StoryObj } from '@storybook/svelte'
import Button from '$lib/components/atoms/Button.svelte'

const meta: Meta<typeof Button> = {
  component: Button
}

export default meta
type Story = StoryObj<typeof meta>

export const Primary: Story = {
  args: {
    children: 'Hello, world!'
  }
}
