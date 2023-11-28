import os
from PIL import Image

TAB = '    '

levels = os.listdir('assets/levels')
levels = [lvl for lvl in levels if lvl.endswith('png')]

out = f'''
#[derive(Clone)]
pub struct Star {{
    pub x: f32,
    pub y: f32,
    pub vert: bool,
}}

#[derive(Clone)]
pub struct Rock {{
    pub x: f32,
    pub y: f32,
    pub mine: bool,
}}

pub const MAX_LEVEL: usize = {len(levels)};

'''

for i, level in enumerate(levels):
    l = i + 1
    img = Image.open(os.path.join('assets', 'levels', level))

    start_i, max_i, start_p, max_p, time_limit = [int(x.replace('.png', '')) for x in level.split('_')[2:]]

    stars = []
    rocks = []
    for x in range(img.size[0]):
        for y in range(img.size[1]):
            pix = '#' + ''.join([f'{c:02x}' for c in img.getpixel((x, y))[:3]])
            xx = (x * 50) - 2500
            yy = 2500 - (y * 50)
            match pix:
                case '#00ff00':
                    stars.append(f'Star {{ x: {xx:.1f}, y: {yy:.1f}, vert: false }},')
                case '#ffff00':
                    stars.append(f'Star {{ x: {xx:.1f}, y: {yy:.1f}, vert: true }},')
                case '#ff0000':
                    rocks.append(f'Rock {{ x: {xx:.1f}, y: {yy:.1f}, mine: false }},')
                case '#0000ff':
                    rocks.append(f'Rock {{ x: {xx:.1f}, y: {yy:.1f}, mine: true }},')

    out += f'//\n// LEVEL {l}\n//\n'
    out += f'pub struct Level{l} {{\n'
    out += f'{TAB}pub stars: [Star; {len(stars)}],\n'
    out += f'{TAB}pub rocks: [Rock; {len(rocks)}],\n'
    out += f'{TAB}pub start_i: usize,\n'
    out += f'{TAB}pub max_i: usize,\n'
    out += f'{TAB}pub start_p: usize,\n'
    out += f'{TAB}pub max_p: usize,\n'
    out += f'{TAB}pub time_limit: usize,\n'
    out += '}\n\n'

    out += f'pub const LEVEL_{l}: Level{l} = Level{l} {{\n'
    
    if stars:
        out += f'{TAB}stars: [\n'
        for star in stars:
            out += f'{TAB * 2}{star}\n'
        out += f'{TAB}],\n'
    else:
        out += f'{TAB}stars: [],\n'

    if rocks:
        out += f'{TAB}rocks: [\n'
        for rock in rocks:
            out += f'{TAB * 2}{rock}\n'
        out += f'{TAB}],\n'
    else:
        out += f'{TAB}rocks: [],\n'
    
    out += f'{TAB}start_i: {start_i},\n'
    out += f'{TAB}max_i: {max_i},\n'
    out += f'{TAB}start_p: {start_p},\n'
    out += f'{TAB}max_p: {max_p},\n'
    out += f'{TAB}time_limit: {time_limit},\n'

    out += '};\n\n\n'

with open('src/levels.rs', 'w') as f:
    f.write(out)