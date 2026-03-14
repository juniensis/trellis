# `trellis_ui`

A stripped-down, Iced-inspired terminal user interface library.

## Architecture

Regions --are positioned into-> Elements --which are composed onto the->
Viewport

Widgets --render to-> Regions

Renderers --render Elements onto-> Viewport

Scatters --either compose on to-> Regions
                               +-or straight onto-> Viewports
