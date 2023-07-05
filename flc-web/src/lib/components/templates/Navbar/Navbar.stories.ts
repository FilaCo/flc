import type { Meta, StoryObj } from '@storybook/svelte'

import Navbar from './Navbar.svelte'

const meta: Meta<typeof Navbar> = {
  component: Navbar,
}

export default meta
type Story = StoryObj<typeof meta>

export const Primary: Story = {
  render: (args) => ({
    Component: Navbar,
    props: args,
  }),

  args: {
    sections: [
      {
        id: '#intro',
        name: 'Intro',
      },
      {
        id: '#projects',
        name: 'Projects',
      },
      {
        id: '#experience',
        name: 'Experience',
      },
      {
        id: '#contact-me',
        name: 'Contact me',
      },
    ],
    mode: 'scroll',
  },
}
