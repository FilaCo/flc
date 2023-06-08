import React from 'react'
import { Roboto_Flex } from 'next/font/google'
import Neo from '@/app/containers/Neo'

const robotoFlex = Roboto_Flex({
  subsets: ['cyrillic', 'latin'],
  display: 'swap',
})

interface LayoutProps {
  about: React.ReactNode
}

const Layout = (props: LayoutProps) => {
  const { about } = props

  return (
    <html lang="en" className={robotoFlex.className}>
      <body>{about}</body>
    </html>
  )
}

export default Layout
