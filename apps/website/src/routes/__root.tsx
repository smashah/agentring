import { createRootRoute, HeadContent, Outlet, Scripts } from '@tanstack/react-router';
import * as React from 'react';
import appCss from '@/styles/app.css?url';

export const Route = createRootRoute({
  head: () => ({
    meta: [
      {
        charSet: 'utf-8',
      },
      {
        name: 'viewport',
        content: 'width=device-width, initial-scale=1',
      },
      {
        title: 'Agent Ring — remap a Bluetooth finger-ring into keyboard shortcuts',
      },
      {
        name: 'description',
        content:
          'Agent Ring is a lightweight native app that captures the HID reports of a WX02-class Bluetooth finger-ring, classifies each gesture, and injects the keyboard shortcut you mapped. macOS first, Windows 11 planned.',
      },
      {
        name: 'color-scheme',
        content: 'dark',
      },
    ],
    links: [{ rel: 'stylesheet', href: appCss }],
  }),
  component: RootComponent,
});

function RootComponent() {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        <HeadContent />
      </head>
      <body>
        <Outlet />
        <Scripts />
      </body>
    </html>
  );
}