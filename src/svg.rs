// Alien logic, don't touch it!

use crate::common::{
    Ingredient, RICE_COLOR, SushiData, SushiShape, SushiState, SushiView, TopEdgeType,
    describe_arc, ingredient_meta, ingredient_texture_id,
};

fn ss(pairs: &[(&str, &str)], t: &str) -> String {
    pairs
        .iter()
        .fold(t.to_string(), |acc, &(k, v)| acc.replace(k, v))
}

fn defs(uid: &str, rice: &str, nori: &str) -> String {
    ss(&[("{U}", uid), ("{R}", rice), ("{N}", nori)],
        "<defs>\
        <radialGradient id='nori-g-{U}' cx='35%' cy='35%' r='70%'>\
        <stop offset='0%' stop-color='#3a6a20'/>\
        <stop offset='55%' stop-color='{N}'/>\
        <stop offset='100%' stop-color='#060e04'/></radialGradient>\
        <radialGradient id='rice-g-{U}' cx='38%' cy='32%' r='68%'>\
        <stop offset='0%' stop-color='#f5f1e6'/>\
        <stop offset='55%' stop-color='{R}'/>\
        <stop offset='100%' stop-color='#c8bfa0'/></radialGradient>\
        <radialGradient id='rice-flat-{U}' cx='50%' cy='50%' r='60%'>\
        <stop offset='0%' stop-color='#f8f5ec'/>\
        <stop offset='100%' stop-color='{R}'/></radialGradient>\
        <pattern id='nori-p-{U}' x='0' y='0' width='6' height='6' patternUnits='userSpaceOnUse'>\
        <line x1='0' y1='6' x2='6' y2='0' stroke='rgba(255,255,255,0.06)' stroke-width='0.9'/>\
        <line x1='-2' y1='4' x2='2' y2='0' stroke='rgba(255,255,255,0.03)' stroke-width='0.6'/></pattern>\
        <pattern id='rice-p-{U}' x='0' y='0' width='8' height='8' patternUnits='userSpaceOnUse'>\
        <ellipse cx='1' cy='1.5' rx='2.0' ry='0.78' transform='rotate(-18,1,1.5)' fill='rgba(255,255,255,0.95)'/>\
        <ellipse cx='4.6' cy='0.7' rx='1.9' ry='0.74' transform='rotate(8,4.6,0.7)' fill='rgba(252,252,248,0.92)'/>\
        <ellipse cx='7' cy='1.8' rx='1.8' ry='0.72' transform='rotate(-28,7,1.8)' fill='rgba(255,255,255,0.94)'/>\
        <ellipse cx='2.4' cy='3.5' rx='2.0' ry='0.78' transform='rotate(22,2.4,3.5)' fill='rgba(250,250,244,0.91)'/>\
        <ellipse cx='6' cy='3.2' rx='1.9' ry='0.75' transform='rotate(-8,6,3.2)' fill='rgba(255,255,255,0.93)'/>\
        <ellipse cx='4' cy='5.9' rx='2.1' ry='0.80' transform='rotate(-16,4,5.9)' fill='rgba(252,252,248,0.94)'/>\
        <ellipse cx='1.9' cy='7.5' rx='2.0' ry='0.78' transform='rotate(-4,1.9,7.5)' fill='rgba(255,255,255,0.93)'/></pattern>\
        <pattern id='salmon-p-{U}' x='0' y='0' width='14' height='14' patternUnits='userSpaceOnUse'>\
        <line x1='0' y1='5' x2='14' y2='9' stroke='rgba(255,185,145,0.36)' stroke-width='1.8'/>\
        <line x1='0' y1='10' x2='14' y2='14' stroke='rgba(225,110,90,0.22)' stroke-width='1.2'/>\
        <line x1='2' y1='0' x2='14' y2='4' stroke='rgba(255,205,170,0.28)' stroke-width='1.1'/></pattern>\
        <pattern id='sheen-p-{U}' x='0' y='0' width='12' height='12' patternUnits='userSpaceOnUse'>\
        <line x1='0' y1='12' x2='12' y2='0' stroke='rgba(255,255,255,0.15)' stroke-width='1.4'/>\
        <line x1='0' y1='7' x2='7' y2='0' stroke='rgba(255,255,255,0.08)' stroke-width='0.9'/></pattern>\
        <pattern id='sesame-p-{U}' x='0' y='0' width='8' height='8' patternUnits='userSpaceOnUse'>\
        <ellipse cx='2' cy='2' rx='1.5' ry='0.65' transform='rotate(-30,2,2)' fill='rgba(240,220,150,0.88)'/>\
        <ellipse cx='6' cy='5' rx='1.4' ry='0.60' transform='rotate(15,6,5)' fill='rgba(220,195,120,0.80)'/>\
        <ellipse cx='1.5' cy='6.5' rx='1.3' ry='0.60' transform='rotate(45,1.5,6.5)' fill='rgba(240,220,150,0.84)'/></pattern>\
        <pattern id='rcav-p-{U}' x='0' y='0' width='7' height='7' patternUnits='userSpaceOnUse'>\
        <circle cx='2' cy='2' r='1.7' fill='rgba(200,50,40,0.65)'/>\
        <circle cx='5.5' cy='5' r='1.6' fill='rgba(180,30,30,0.60)'/>\
        <circle cx='1' cy='5.5' r='1.3' fill='rgba(200,60,50,0.58)'/>\
        <circle cx='1.6' cy='1.5' r='0.5' fill='rgba(255,180,160,0.55)'/></pattern>\
        <pattern id='bcav-p-{U}' x='0' y='0' width='7' height='7' patternUnits='userSpaceOnUse'>\
        <circle cx='2' cy='2' r='1.7' fill='rgba(25,25,25,0.75)'/>\
        <circle cx='5.5' cy='5' r='1.6' fill='rgba(40,40,40,0.70)'/>\
        <circle cx='1' cy='5.5' r='1.3' fill='rgba(30,30,30,0.65)'/>\
        <circle cx='1.6' cy='1.5' r='0.5' fill='rgba(120,120,120,0.50)'/></pattern>\
        <pattern id='tobiko-p-{U}' x='0' y='0' width='6' height='6' patternUnits='userSpaceOnUse'>\
        <circle cx='1.5' cy='1.5' r='1.3' fill='rgba(255,130,50,0.70)'/>\
        <circle cx='4.5' cy='4' r='1.2' fill='rgba(220,100,30,0.65)'/>\
        <circle cx='1.2' cy='1.1' r='0.45' fill='rgba(255,210,160,0.55)'/></pattern>\
        </defs>")
    .replace('\'', "\"")
}

fn rice_ring_dots(cx: f64, cy: f64, r: f64, count: usize) -> String {
    (0..count).map(|i| {
        let a = (i as f64 / count as f64) * 2.0 * std::f64::consts::PI;
        let ex = cx + r * a.cos();
        let ey = cy + r * a.sin();
        let rot = (i as f64 / count as f64) * 360.0;
        format!("<ellipse cx='{ex:.2}' cy='{ey:.2}' rx='2' ry='1.1' transform='rotate({rot:.1},{ex:.2},{ey:.2})' fill='rgba(255,255,255,0.40)'/>")
    }).collect::<Vec<_>>().join("").replace('\'', "\"")
}

fn ingred_circle(ingredients: &[Ingredient], cx: f64, cy: f64, r: f64, uid: &str) -> String {
    if ingredients.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    if ingredients.len() == 1 {
        let m = ingredient_meta(ingredients[0]);
        let tex = ingredient_texture_id(ingredients[0], uid);
        out.push_str(&format!(
            "<g role='img' aria-label='{lb}'><title>{lb}</title>\
            <circle cx='{cx}' cy='{cy}' r='{r}' fill='{f}'/>\
            <circle cx='{cx}' cy='{cy}' r='{r}' fill='{t}' opacity='0.7'/>\
            <circle cx='{hx:.1}' cy='{hy:.1}' r='{hr:.1}' fill='rgba(255,255,255,0.2)'/></g>",
            lb = m.label,
            cx = cx,
            cy = cy,
            r = r,
            f = m.fill,
            t = tex,
            hx = cx - r * 0.25,
            hy = cy - r * 0.22,
            hr = r * 0.28
        ));
    } else {
        let sl = 360.0 / ingredients.len() as f64;
        for (i, &ing) in ingredients.iter().enumerate() {
            let m = ingredient_meta(ing);
            let tex = ingredient_texture_id(ing, uid);
            let arc = describe_arc(cx, cy, r, i as f64 * sl, (i + 1) as f64 * sl);
            out.push_str(&format!(
                "<g role='img' aria-label='{lb}'><title>{lb}</title>\
                <path d='{arc}' fill='{f}'/>\
                <path d='{arc}' fill='{t}' opacity='0.55'/></g>",
                lb = m.label,
                arc = arc,
                f = m.fill,
                t = tex
            ));
        }
    }
    out.replace('\'', "\"")
}

fn ingred_rect(ingredients: &[Ingredient], cx: f64, cy: f64, half: f64, uid: &str) -> String {
    if ingredients.is_empty() {
        return String::new();
    }
    let x = cx - half;
    let y = cy - half;
    let s = half * 2.0;
    let clip = format!("sq-ing-clip-{uid}");
    let mut out = format!(
        "<defs><clipPath id='{clip}'><rect x='{x}' y='{y}' width='{s}' height='{s}' rx='4'/></clipPath></defs>",
        clip = clip,
        x = x,
        y = y,
        s = s
    );
    if ingredients.len() == 1 {
        let m = ingredient_meta(ingredients[0]);
        let tex = ingredient_texture_id(ingredients[0], uid);
        out.push_str(&format!(
            "<g role='img' aria-label='{lb}'><title>{lb}</title>\
            <rect x='{x}' y='{y}' width='{s}' height='{s}' rx='4' fill='{f}'/>\
            <rect x='{x}' y='{y}' width='{s}' height='{s}' rx='4' fill='{t}' opacity='0.6'/></g>",
            lb = m.label,
            x = x,
            y = y,
            s = s,
            f = m.fill,
            t = tex
        ));
    } else {
        let sl = 360.0 / ingredients.len() as f64;
        for (i, &ing) in ingredients.iter().enumerate() {
            let m = ingredient_meta(ing);
            let tex = ingredient_texture_id(ing, uid);
            let arc = describe_arc(cx, cy, half * 1.5, i as f64 * sl, (i + 1) as f64 * sl);
            out.push_str(&format!(
                "<g role='img' aria-label='{lb}'><title>{lb}</title>\
                <path d='{arc}' fill='{f}' clip-path='url(#{clip})'/>\
                <path d='{arc}' fill='{t}' opacity='0.5' clip-path='url(#{clip})'/></g>",
                lb = m.label,
                arc = arc,
                f = m.fill,
                t = tex,
                clip = clip
            ));
        }
    }
    out.replace('\'', "\"")
}

fn ingred_tri(
    ingredients: &[Ingredient],
    pts: &str,
    cx: f64,
    cy: f64,
    r: f64,
    uid: &str,
) -> String {
    if ingredients.is_empty() {
        return String::new();
    }
    let clip = format!("tri-ing-clip-{uid}");
    let mut out = format!(
        "<defs><clipPath id='{clip}'><polygon points='{pts}'/></clipPath></defs>",
        clip = clip,
        pts = pts
    );
    if ingredients.len() == 1 {
        let m = ingredient_meta(ingredients[0]);
        let tex = ingredient_texture_id(ingredients[0], uid);
        out.push_str(&format!(
            "<g role='img' aria-label='{lb}'><title>{lb}</title>\
            <polygon points='{pts}' fill='{f}'/>\
            <polygon points='{pts}' fill='{t}' opacity='0.55'/>\
            <circle cx='{cx}' cy='{cy}' r='{cr:.1}' fill='rgba(255,255,255,0.16)'/></g>",
            lb = m.label,
            pts = pts,
            f = m.fill,
            t = tex,
            cx = cx,
            cy = cy,
            cr = r * 0.45
        ));
    } else {
        let sl = 360.0 / ingredients.len() as f64;
        for (i, &ing) in ingredients.iter().enumerate() {
            let m = ingredient_meta(ing);
            let tex = ingredient_texture_id(ing, uid);
            let arc = describe_arc(cx, cy, r * 2.0, i as f64 * sl, (i + 1) as f64 * sl);
            out.push_str(&format!(
                "<g role='img' aria-label='{lb}'><title>{lb}</title>\
                <path d='{arc}' fill='{f}' clip-path='url(#{clip})'/>\
                <path d='{arc}' fill='{t}' opacity='0.5' clip-path='url(#{clip})'/></g>",
                lb = m.label,
                arc = arc,
                f = m.fill,
                t = tex,
                clip = clip
            ));
        }
    }
    out.replace('\'', "\"")
}

fn top_edge_ring(
    uid: &str,
    cx: f64,
    cy: f64,
    outer_r: f64,
    inner_r: f64,
    edge_type: TopEdgeType,
    color: Option<&str>,
) -> String {
    if edge_type == TopEdgeType::None {
        return String::new();
    }
    let clip = format!("tedge-clip-{uid}");
    let or = outer_r;
    let ir = inner_r;
    let clip_def = format!(
        "<defs><clipPath id='{clip}'><path d='M {cx} {cy} m -{or} 0 a {or} {or} 0 1 0 {dor} 0 a {or} {or} 0 1 0 -{dor} 0 M {cx} {cy} m -{ir} 0 a {ir} {ir} 0 1 1 {dir} 0 a {ir} {ir} 0 1 1 -{dir} 0' fill-rule='evenodd'/></clipPath></defs>",
        clip = clip, cx = cx, cy = cy, or = or, dor = or * 2.0, ir = ir, dir = ir * 2.0
    ).replace('\'', "\"");
    let pat_id = match edge_type {
        TopEdgeType::RedCaviar => Some(format!("rcav-p-{uid}")),
        TopEdgeType::BlackCaviar => Some(format!("bcav-p-{uid}")),
        TopEdgeType::Tobiko => Some(format!("tobiko-p-{uid}")),
        TopEdgeType::Sesame => Some(format!("sesame-p-{uid}")),
        _ => None,
    };
    if let Some(pat) = pat_id {
        format!(
            "{clip_def}<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{outer_r}\" fill=\"url(#{pat})\" clip-path=\"url(#{clip})\" opacity=\"0.9\"/>"
        )
    } else {
        let herb_color = color.unwrap_or("#4a8a3a");
        let herbs: String = (0..16).map(|i| {
            let ang = (i as f64 / 16.0) * 2.0 * std::f64::consts::PI;
            let rr = (outer_r + inner_r) / 2.0;
            let (ex, ey) = (cx + rr * ang.cos(), cy + rr * ang.sin());
            let rot = (i as f64 / 16.0) * 360.0;
            format!("<ellipse cx=\"{ex:.2}\" cy=\"{ey:.2}\" rx=\"2\" ry=\"1\" transform=\"rotate({rot:.1},{ex:.2},{ey:.2})\" fill=\"{herb_color}\" opacity=\"0.85\"/>")
        }).collect();
        format!("{clip_def}{herbs}")
    }
}

fn wrap_svg(w: f64, h: f64, label: &str, inner: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w}\" height=\"{h}\" viewBox=\"0 0 {w} {h}\" role=\"img\" aria-label=\"{label}\">{inner}</svg>"
    )
}

fn circular_top(d: &SushiData, uid: &str) -> String {
    let rice = d.rice_color.as_deref().unwrap_or(RICE_COLOR);
    let nori = d.outer_sheet.color.as_str();
    let w = d.size.width;
    let h = d.size.height;
    let cx = w / 2.0;
    let cy = h / 2.0;
    let outer_r = w.min(h) / 2.0 - 2.0;
    let rice_r = outer_r - d.outer_sheet.thickness;
    let ing_r = rice_r * 0.50;
    let edge = top_edge_ring(
        uid,
        cx,
        cy,
        outer_r,
        outer_r - 2.0,
        d.top_edge
            .as_ref()
            .map(|e| e.edge_type)
            .unwrap_or(TopEdgeType::None),
        d.top_edge.as_ref().and_then(|e| e.color.as_deref()),
    );
    let inner = format!(
        "{d}\
        <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{or}\" fill=\"url(#nori-g-{u})\"/>\
        <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{or}\" fill=\"url(#nori-p-{u})\" opacity=\"0.6\"/>\
        <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{rr}\" fill=\"url(#rice-g-{u})\"/>\
        <circle cx=\"{cx}\" cy=\"{cy}\" r=\"{rr}\" fill=\"url(#rice-p-{u})\" opacity=\"1.0\"/>\
        {dots}{ings}{edge}",
        d = defs(uid, rice, nori),
        cx = cx,
        cy = cy,
        u = uid,
        or = outer_r,
        rr = rice_r,
        dots = rice_ring_dots(cx, cy, rice_r * 0.76, 20),
        ings = ingred_circle(&d.ingredients, cx, cy, ing_r, uid),
        edge = edge,
    );
    wrap_svg(w, h, &format!("{} top view", d.name), &inner)
}

fn square_top(d: &SushiData, uid: &str) -> String {
    let rice = d.rice_color.as_deref().unwrap_or(RICE_COLOR);
    let nori = d.outer_sheet.color.as_str();
    let w = d.size.width;
    let h = d.size.height;
    let cx = w / 2.0;
    let cy = h / 2.0;
    let pad = 3.0;
    let cr = 12.0;
    let thick = d.outer_sheet.thickness;
    let ix = pad + thick;
    let iy = pad + thick;
    let iw = w - 2.0 * (pad + thick);
    let ih = h - 2.0 * (pad + thick);
    let half = iw.min(ih) * 0.27;
    let inner = format!(
        "{d}\
        <rect x=\"{pad}\" y=\"{pad}\" width=\"{nw}\" height=\"{nh}\" rx=\"{cr}\" fill=\"url(#nori-g-{u})\"/>\
        <rect x=\"{pad}\" y=\"{pad}\" width=\"{nw}\" height=\"{nh}\" rx=\"{cr}\" fill=\"url(#nori-p-{u})\" opacity=\"0.5\"/>\
        <rect x=\"{ix}\" y=\"{iy}\" width=\"{iw}\" height=\"{ih}\" rx=\"{irx}\" fill=\"url(#rice-g-{u})\"/>\
        <rect x=\"{ix}\" y=\"{iy}\" width=\"{iw}\" height=\"{ih}\" rx=\"{irx}\" fill=\"url(#rice-p-{u})\" opacity=\"1.0\"/>\
        {dots}{ings}",
        d = defs(uid, rice, nori),
        u = uid,
        pad = pad,
        nw = w - 2.0 * pad,
        nh = h - 2.0 * pad,
        cr = cr,
        ix = ix,
        iy = iy,
        iw = iw,
        ih = ih,
        irx = cr - 3.0,
        dots = rice_ring_dots(cx, cy, iw.min(ih) * 0.40, 16),
        ings = ingred_rect(&d.ingredients, cx, cy, half, uid),
    );
    wrap_svg(w, h, &format!("{} top view", d.name), &inner)
}

fn triangular_top(d: &SushiData, uid: &str) -> String {
    let rice = d.rice_color.as_deref().unwrap_or(RICE_COLOR);
    let nori = d.outer_sheet.color.as_str();
    let w = d.size.width;
    let h = d.size.height;
    let pad = 5.0;
    let outer_pts = format!(
        "{},{} {},{} {},{}",
        w / 2.0,
        pad,
        pad,
        h - pad,
        w - pad,
        h - pad
    );
    let t = d.outer_sheet.thickness * 1.2;
    let ra = (w / 2.0, pad + t * 1.8);
    let rb = (pad + t, h - pad - t * 0.7);
    let rc = (w - pad - t, h - pad - t * 0.7);
    let rice_pts = format!("{},{} {},{} {},{}", ra.0, ra.1, rb.0, rb.1, rc.0, rc.1);
    let rcx = (ra.0 + rb.0 + rc.0) / 3.0;
    let rcy = (ra.1 + rb.1 + rc.1) / 3.0;
    let sc = 0.38_f64;
    let ing_pts = format!(
        "{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
        rcx + (ra.0 - rcx) * sc,
        rcy + (ra.1 - rcy) * sc,
        rcx + (rb.0 - rcx) * sc,
        rcy + (rb.1 - rcy) * sc,
        rcx + (rc.0 - rcx) * sc,
        rcy + (rc.1 - rcy) * sc
    );
    let ing_r = w.min(h) * 0.18;
    let nori_y = h * 0.60;
    let clip = format!("tri-clip-{uid}");
    let bot_h = h - nori_y + pad;
    let inner = format!(
        "{d}\
        <defs><clipPath id=\"{clip}\"><polygon points=\"{op}\"/></clipPath></defs>\
        <polygon points=\"{op}\" fill=\"url(#rice-g-{u})\"/>\
        <polygon points=\"{op}\" fill=\"url(#rice-p-{u})\" opacity=\"1.0\"/>\
        <rect x=\"0\" y=\"{ny}\" width=\"{w}\" height=\"{bh}\" fill=\"{n}\" clip-path=\"url(#{clip})\"/>\
        <rect x=\"0\" y=\"{ny}\" width=\"{w}\" height=\"{bh}\" fill=\"url(#nori-p-{u})\" opacity=\"0.45\" clip-path=\"url(#{clip})\"/>\
        <polygon points=\"{op}\" fill=\"none\" stroke=\"{n}\" stroke-width=\"{tk}\"/>\
        <polygon points=\"{rp}\" fill=\"url(#rice-g-{u})\"/>\
        <polygon points=\"{rp}\" fill=\"url(#rice-p-{u})\" opacity=\"1.0\"/>\
        {ings}{dots}",
        d = defs(uid, rice, nori),
        u = uid,
        clip = clip,
        op = outer_pts,
        rp = rice_pts,
        n = nori,
        ny = nori_y,
        bh = bot_h,
        w = w,
        tk = d.outer_sheet.thickness,
        ings = ingred_tri(&d.ingredients, &ing_pts, rcx, rcy, ing_r, uid),
        dots = rice_ring_dots(rcx, rcy, ing_r * 1.8, 14),
    );
    wrap_svg(w, h, &format!("{} top view", d.name), &inner)
}

fn oval_top(d: &SushiData, uid: &str) -> String {
    let rice = d.rice_color.as_deref().unwrap_or(RICE_COLOR);
    let nori = d.outer_sheet.color.as_str();
    let w = d.size.width;
    let h = d.size.height;
    let cx = w / 2.0;
    let cy = h / 2.0;
    let rice_rx = w * 0.42;
    let rice_ry = h * 0.25;
    let top_rx = rice_rx * 0.82;
    let top_ry = rice_ry * 0.60;
    let clip = format!("ovaltop-clip-{uid}");
    let n = d.ingredients.len().max(1) as f64;
    let slice_w = top_rx * 2.0 / n;
    let ing_slices: String = d.ingredients.iter().enumerate().map(|(i, &ing)| {
        let m = ingredient_meta(ing);
        let tex = ingredient_texture_id(ing, uid);
        let x = cx - top_rx + i as f64 * slice_w;
        format!(
            "<g role=\"img\" aria-label=\"{lb}\" clip-path=\"url(#{clip})\"><title>{lb}</title>\
            <rect x=\"{x:.2}\" y=\"{ty:.2}\" width=\"{sw:.2}\" height=\"{th:.2}\" fill=\"{f}\" opacity=\"0.94\"/>\
            <rect x=\"{x:.2}\" y=\"{ty:.2}\" width=\"{sw:.2}\" height=\"{th:.2}\" fill=\"{t}\" opacity=\"1.0\"/></g>",
            lb = m.label, clip = clip, x = x, ty = cy - top_ry, th = top_ry * 2.0,
            sw = slice_w, f = m.fill, t = tex
        )
    }).collect();
    let nori_band = if nori != "transparent" && d.outer_sheet.thickness > 0.0 {
        let bw = (d.outer_sheet.thickness * 1.5).max(4.0);
        format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"{n}\" opacity=\"0.9\"/>",
            cx - bw,
            cy - rice_ry,
            bw * 2.0,
            rice_ry * 2.0,
            n = nori
        )
    } else {
        String::new()
    };
    let inner = format!(
        "{d}\
        <defs><clipPath id=\"{clip}\"><ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{trx}\" ry=\"{try_}\"/></clipPath></defs>\
        <ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rrx}\" ry=\"{rry}\" fill=\"url(#rice-g-{u})\"/>\
        <ellipse cx=\"{cx}\" cy=\"{cy}\" rx=\"{rrx}\" ry=\"{rry}\" fill=\"url(#rice-p-{u})\" opacity=\"1.0\"/>\
        {dots}{ings}{nb}",
        d = defs(uid, rice, nori),
        cx = cx,
        cy = cy,
        u = uid,
        clip = clip,
        trx = top_rx,
        try_ = top_ry,
        rrx = rice_rx,
        rry = rice_ry,
        dots = rice_ring_dots(cx, cy, (rice_rx + rice_ry) / 2.0 * 0.7, 18),
        ings = ing_slices,
        nb = nori_band,
    );
    wrap_svg(w, h, &format!("{} top view", d.name), &inner)
}

fn circular_front(d: &SushiData, uid: &str) -> String {
    let rice = d.rice_color.as_deref().unwrap_or(RICE_COLOR);
    let nori = d.outer_sheet.color.as_str();
    let w = d.size.width;
    let h = d.size.height;
    let cx = w / 2.0;
    let cyl_h = h * 0.60;
    let top_y = (h - cyl_h) / 2.0;
    let bot_y = top_y + cyl_h;
    let rx = w / 2.0 - 4.0;
    let ry = rx * 0.50;
    let thick = d.outer_sheet.thickness;
    let ing_rx = (rx - thick) * 0.72;
    let ing_ry = ((ry - thick * 0.5) * 0.72).max(1.0);
    let sl = if !d.ingredients.is_empty() {
        360.0 / d.ingredients.len() as f64
    } else {
        0.0
    };
    let ing_html: String = if d.ingredients.len() == 1 {
        let m = ingredient_meta(d.ingredients[0]);
        let tex = ingredient_texture_id(d.ingredients[0], uid);
        format!(
            "<g role=\"img\" aria-label=\"{lb}\"><title>{lb}</title>\
            <ellipse cx=\"{cx}\" cy=\"{ty}\" rx=\"{irx}\" ry=\"{iry}\" fill=\"{f}\" opacity=\"0.93\"/>\
            <ellipse cx=\"{cx}\" cy=\"{ty}\" rx=\"{irx}\" ry=\"{iry}\" fill=\"{t}\" opacity=\"1.0\"/></g>",
            lb = m.label,
            cx = cx,
            ty = top_y,
            irx = ing_rx,
            iry = ing_ry,
            f = m.fill,
            t = tex
        )
    } else {
        d.ingredients.iter().enumerate().map(|(i, &ing)| {
            let m = ingredient_meta(ing);
            let tex = ingredient_texture_id(ing, uid);
            let a1 = ((i as f64 * sl - 90.0) * std::f64::consts::PI) / 180.0;
            let a2 = (((i + 1) as f64 * sl - 90.0) * std::f64::consts::PI) / 180.0;
            let (x1, y1) = (cx + ing_rx * a1.cos(), top_y + ing_ry * a1.sin());
            let (x2, y2) = (cx + ing_rx * a2.cos(), top_y + ing_ry * a2.sin());
            let large = if sl > 180.0 { 1 } else { 0 };
            let arc = format!("M {cx} {top_y} L {x1:.2} {y1:.2} A {ing_rx} {ing_ry} 0 {large} 1 {x2:.2} {y2:.2} Z");
            format!(
                "<g role=\"img\" aria-label=\"{lb}\"><title>{lb}</title>\
                <path d=\"{arc}\" fill=\"{f}\" opacity=\"0.93\"/>\
                <path d=\"{arc}\" fill=\"{t}\" opacity=\"0.5\"/></g>",
                lb = m.label, arc = arc, f = m.fill, t = tex
            )
        }).collect()
    };
    let lx = cx - rx;
    let rx2 = cx + rx;
    let irx = (rx - thick).max(1.0);
    let iry = (ry - thick * 0.5).max(1.0);
    let grad = format!(
        "<defs><linearGradient id=\"cyl-side-{uid}\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\
        <stop offset=\"0%\" stop-color=\"#060e04\"/>\
        <stop offset=\"18%\" stop-color=\"#2a5a18\"/>\
        <stop offset=\"50%\" stop-color=\"{n}\"/>\
        <stop offset=\"82%\" stop-color=\"#1a3a0a\"/>\
        <stop offset=\"100%\" stop-color=\"#060e04\"/></linearGradient></defs>",
        uid = uid,
        n = nori
    );
    let inner = format!(
        "{d}{g}\
        <ellipse cx=\"{cx}\" cy=\"{by}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"rgba(0,0,0,0.25)\"/>\
        <path d=\"M {lx:.1} {ty} L {lx:.1} {by} A {rx} {ry} 0 0 0 {rx2:.1} {by} L {rx2:.1} {ty} Z\" fill=\"url(#cyl-side-{u})\"/>\
        <path d=\"M {lx:.1} {ty} L {lx:.1} {by} A {rx} {ry} 0 0 0 {rx2:.1} {by} L {rx2:.1} {ty} Z\" fill=\"url(#nori-p-{u})\" opacity=\"0.4\"/>\
        <ellipse cx=\"{cx}\" cy=\"{ty}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"url(#nori-g-{u})\"/>\
        <ellipse cx=\"{cx}\" cy=\"{ty}\" rx=\"{rx}\" ry=\"{ry}\" fill=\"url(#nori-p-{u})\" opacity=\"0.5\"/>\
        <ellipse cx=\"{cx}\" cy=\"{ty}\" rx=\"{irx}\" ry=\"{iry}\" fill=\"url(#rice-flat-{u})\"/>\
        <ellipse cx=\"{cx}\" cy=\"{ty}\" rx=\"{irx}\" ry=\"{iry}\" fill=\"url(#rice-p-{u})\" opacity=\"1.0\"/>\
        {ih}",
        d = defs(uid, rice, nori),
        g = grad,
        u = uid,
        cx = cx,
        ty = top_y,
        by = bot_y,
        rx = rx,
        ry = ry,
        lx = lx,
        rx2 = rx2,
        irx = irx,
        iry = iry,
        ih = ing_html,
    );
    wrap_svg(w, h, &format!("{} front view", d.name), &inner)
}

fn square_front(d: &SushiData, uid: &str) -> String {
    let rice = d.rice_color.as_deref().unwrap_or(RICE_COLOR);
    let nori = d.outer_sheet.color.as_str();
    let w = d.size.width;
    let h = d.size.height;
    let box_h = h * 0.58;
    let top_y = (h - box_h) / 2.0;
    let thick = d.outer_sheet.thickness;
    let iso = box_h * 0.22;
    let cw = w - 8.0;
    let rx1 = 4.0 + thick * 1.5;
    let rx2 = 4.0 + cw - thick * 1.5;
    let ly1 = top_y + iso - thick * 0.6;
    let ly2 = top_y + thick * 0.6;
    let top_pts = format!(
        "4,{:.1} {:.1},{:.1} {:.1},{:.1} 4,{:.1}",
        top_y + iso,
        4.0 + cw,
        top_y + iso,
        4.0 + cw,
        top_y,
        top_y
    );
    let rice_pts = format!(
        "{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
        rx1, ly1, rx2, ly1, rx2, ly2, rx1, ly2
    );
    let ix1 = rx1 + (rx2 - rx1) * 0.25;
    let ix2 = rx2 - (rx2 - rx1) * 0.25;
    let iy1 = ly1 - (ly1 - ly2) * 0.25;
    let iy2 = ly2 + (ly1 - ly2) * 0.25;
    let ing_html = if let Some(&ing) = d.ingredients.first() {
        let m = ingredient_meta(ing);
        let tex = ingredient_texture_id(ing, uid);
        let ip = format!(
            "{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
            ix1, iy1, ix2, iy1, ix2, iy2, ix1, iy2
        );
        format!(
            "<g role=\"img\" aria-label=\"{lb}\"><title>{lb}</title>\
            <polygon points=\"{ip}\" fill=\"{f}\" opacity=\"0.92\"/>\
            <polygon points=\"{ip}\" fill=\"{t}\" opacity=\"1.0\"/></g>",
            lb = m.label,
            ip = ip,
            f = m.fill,
            t = tex
        )
    } else {
        String::new()
    };
    let grad = format!(
        "<defs><linearGradient id=\"sq-side-{uid}\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\
        <stop offset=\"0%\" stop-color=\"#060e04\"/>\
        <stop offset=\"20%\" stop-color=\"#2a5a18\"/>\
        <stop offset=\"50%\" stop-color=\"{n}\"/>\
        <stop offset=\"80%\" stop-color=\"#1a3a0a\"/>\
        <stop offset=\"100%\" stop-color=\"#060e04\"/></linearGradient></defs>",
        uid = uid,
        n = nori
    );
    let face_y = top_y + iso;
    let inner = format!(
        "{d}{g}\
        <rect x=\"4\" y=\"{fy}\" width=\"{cw}\" height=\"{bh}\" rx=\"4\" fill=\"url(#sq-side-{u})\"/>\
        <rect x=\"4\" y=\"{fy}\" width=\"{cw}\" height=\"{bh}\" rx=\"4\" fill=\"url(#nori-p-{u})\" opacity=\"0.4\"/>\
        <polygon points=\"{tp}\" fill=\"url(#nori-g-{u})\"/>\
        <polygon points=\"{tp}\" fill=\"url(#nori-p-{u})\" opacity=\"0.5\"/>\
        <polygon points=\"{rp}\" fill=\"url(#rice-flat-{u})\"/>\
        <polygon points=\"{rp}\" fill=\"url(#rice-p-{u})\" opacity=\"1.0\"/>\
        {ih}",
        d = defs(uid, rice, nori),
        g = grad,
        u = uid,
        fy = face_y,
        cw = cw,
        bh = box_h,
        tp = top_pts,
        rp = rice_pts,
        ih = ing_html,
    );
    wrap_svg(w, h, &format!("{} front view", d.name), &inner)
}

fn triangular_front(d: &SushiData, uid: &str) -> String {
    let rice = d.rice_color.as_deref().unwrap_or(RICE_COLOR);
    let nori = d.outer_sheet.color.as_str();
    let w = d.size.width;
    let h = d.size.height;
    let box_h = h * 0.54;
    let top_y = (h - box_h) / 2.0;
    let iso = box_h * 0.35;
    let thick = d.outer_sheet.thickness;
    let ap_y = top_y + iso;
    let top_pts = format!("{},{} 12,{} {},{}", w / 2.0, ap_y, top_y, w - 12.0, top_y);
    let rice_pts = format!(
        "{},{} {},{} {},{}",
        w / 2.0,
        ap_y - thick * 0.8,
        12.0 + thick * 1.5,
        top_y + thick * 0.5,
        w - 12.0 - thick * 1.5,
        top_y + thick * 0.5
    );
    let lw = format!(
        "{},{} 12,{} 12,{} {},{}",
        w / 2.0,
        ap_y,
        top_y,
        top_y + box_h,
        w / 2.0,
        ap_y + box_h
    );
    let rw = format!(
        "{},{} {},{} {},{} {},{}",
        w / 2.0,
        ap_y,
        w - 12.0,
        top_y,
        w - 12.0,
        top_y + box_h,
        w / 2.0,
        ap_y + box_h
    );
    let cx = w / 2.0;
    let cy = top_y + iso * 0.55;
    let ing_r = w * 0.12;
    let ing_pts = format!(
        "{},{} {},{} {},{}",
        cx,
        cy + ing_r,
        cx - ing_r,
        cy - ing_r * 0.5,
        cx + ing_r,
        cy - ing_r * 0.5
    );
    let ing_html = if let Some(&ing) = d.ingredients.first() {
        let m = ingredient_meta(ing);
        let tex = ingredient_texture_id(ing, uid);
        format!(
            "<g role=\"img\" aria-label=\"{lb}\"><title>{lb}</title>\
            <polygon points=\"{ip}\" fill=\"{f}\" opacity=\"0.94\"/>\
            <polygon points=\"{ip}\" fill=\"{t}\" opacity=\"1.0\"/></g>",
            lb = m.label,
            ip = ing_pts,
            f = m.fill,
            t = tex
        )
    } else {
        String::new()
    };
    let grad = format!(
        "<defs>\
        <linearGradient id=\"triL-{u}\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\
        <stop offset=\"0%\" stop-color=\"#040b02\"/>\
        <stop offset=\"100%\" stop-color=\"{n}\"/></linearGradient>\
        <linearGradient id=\"triR-{u}\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\">\
        <stop offset=\"0%\" stop-color=\"{n}\"/>\
        <stop offset=\"100%\" stop-color=\"#1a3a0a\"/></linearGradient>\
        </defs>",
        u = uid,
        n = nori
    );
    let inner = format!(
        "{d}{g}\
        <polygon points=\"{lw}\" fill=\"url(#triL-{u})\"/>\
        <polygon points=\"{lw}\" fill=\"url(#nori-p-{u})\" opacity=\"0.4\"/>\
        <polygon points=\"{rw}\" fill=\"url(#triR-{u})\"/>\
        <polygon points=\"{rw}\" fill=\"url(#nori-p-{u})\" opacity=\"0.4\"/>\
        <polygon points=\"{tp}\" fill=\"url(#nori-g-{u})\"/>\
        <polygon points=\"{rp}\" fill=\"url(#rice-flat-{u})\"/>\
        <polygon points=\"{rp}\" fill=\"url(#rice-p-{u})\" opacity=\"1.0\"/>\
        {ih}",
        d = defs(uid, rice, nori),
        g = grad,
        u = uid,
        lw = lw,
        rw = rw,
        tp = top_pts,
        rp = rice_pts,
        ih = ing_html,
    );
    wrap_svg(w, h, &format!("{} front view", d.name), &inner)
}

fn oval_front(d: &SushiData, uid: &str) -> String {
    let rice = d.rice_color.as_deref().unwrap_or(RICE_COLOR);
    let nori = d.outer_sheet.color.as_str();
    let w = d.size.width;
    let h = d.size.height;
    let cx = w / 2.0;
    let body_w = w * 0.32;
    let body_h = h * 0.50;
    let cy = h * 0.55;
    let bot_y = cy + body_h / 2.0;
    let top_y = cy - body_h / 2.0;
    let ry = body_w * 0.4;
    let belt_w = (d.outer_sheet.thickness * 1.4).max(4.0);
    let lx = cx - body_w;
    let rx_ = cx + body_w;
    let sil = format!(
        "M {lx} {top_y} L {lx} {bot_y} A {body_w} {ry} 0 0 0 {rx_} {bot_y} L {rx_} {top_y} A {body_w} {ry} 0 0 0 {lx} {top_y} Z"
    );
    let drape_limit = top_y + body_h * 0.45;
    let n_ings = d.ingredients.len().max(1) as f64;
    let slice_h = (drape_limit - (top_y - ry)) / n_ings;
    let clip_id = format!("ovalfront-sil-{uid}");
    let ing_html: String = d
        .ingredients
        .iter()
        .rev()
        .enumerate()
        .map(|(rev_i, &ing)| {
            let i = d.ingredients.len() - 1 - rev_i;
            let m = ingredient_meta(ing);
            let tex = ingredient_texture_id(ing, uid);
            let end_y = top_y - ry + (i + 1) as f64 * slice_h;
            let path = format!(
                "M {lx} {ty} L {rx_} {ty} L {rx_} {ey} A {body_w} {ry} 0 0 1 {lx} {ey} Z",
                ty = top_y - ry * 2.0,
                ey = end_y
            );
            format!(
                "<g role=\"img\" aria-label=\"{lb}\"><title>{lb}</title>\
            <path d=\"{path}\" fill=\"{f}\" opacity=\"0.93\"/>\
            <path d=\"{path}\" fill=\"{t}\" opacity=\"0.6\"/></g>",
                lb = m.label,
                f = m.fill,
                t = tex
            )
        })
        .collect();
    let belt = if nori != "transparent" && d.outer_sheet.thickness > 0.0 {
        format!(
            "<path d=\"M {lx} {y1} A {body_w} {ry} 0 0 0 {rx_} {y1} L {rx_} {y2} A {body_w} {ry} 0 0 1 {lx} {y2} Z\" fill=\"{n}\" opacity=\"0.93\"/>",
            y1 = cy - belt_w / 2.0,
            y2 = cy + belt_w / 2.0,
            n = nori
        )
    } else {
        String::new()
    };
    let inner = format!(
        "{d}\
        <defs><clipPath id=\"{ci}\"><path d=\"{sil}\"/></clipPath></defs>\
        <ellipse cx=\"{cx}\" cy=\"{sh_y:.1}\" rx=\"{body_w}\" ry=\"{ry}\" fill=\"rgba(0,0,0,0.25)\"/>\
        <path d=\"{sil}\" fill=\"url(#rice-g-{u})\"/>\
        <path d=\"{sil}\" fill=\"url(#rice-p-{u})\" opacity=\"1.0\"/>\
        <g clip-path=\"url(#{ci})\">{ih}</g>{belt}",
        d = defs(uid, rice, nori),
        u = uid,
        ci = clip_id,
        sil = sil,
        cx = cx,
        body_w = body_w,
        ry = ry,
        sh_y = bot_y + ry * 0.8,
        ih = ing_html,
        belt = belt,
    );
    wrap_svg(w, h, &format!("{} front view", d.name), &inner)
}

fn exploded_view(d: &SushiData, uid: &str) -> String {
    let rice = d.rice_color.as_deref().unwrap_or(RICE_COLOR);
    let nori = d.outer_sheet.color.as_str();
    let s = d.size.width.min(d.size.height) * 0.55;
    let sw = s - 4.0;
    let mut parts: Vec<String> = Vec::new();
    if nori != "transparent" {
        let eid = format!("{uid}0");
        let inner_svg = format!(
            "<svg width=\"{s}\" height=\"{s}\" viewBox=\"0 0 {s} {s}\" role=\"img\" aria-label=\"Nori sheet\">{d}\
            <rect x=\"2\" y=\"2\" width=\"{sw}\" height=\"{sw}\" rx=\"8\" fill=\"url(#nori-g-{e})\"/>\
            <rect x=\"2\" y=\"2\" width=\"{sw}\" height=\"{sw}\" rx=\"8\" fill=\"url(#nori-p-{e})\" opacity=\"0.5\"/></svg>",
            s = s,
            sw = sw,
            d = defs(&eid, rice, nori),
            e = eid
        );
        parts.push(format!(
            "<div class=\"sushi-exploded-part\" style=\"animation-delay:0ms\">{inner_svg}<span class=\"sushi-exploded-label\">Nori</span></div>\
            <span class=\"sushi-exploded-sep\" aria-hidden=\"true\">+</span>"
        ));
    }
    let eid1 = format!("{uid}1");
    let inner_rice = format!(
        "<svg width=\"{s}\" height=\"{s}\" viewBox=\"0 0 {s} {s}\" role=\"img\" aria-label=\"Rice\">{d}\
        <rect x=\"2\" y=\"2\" width=\"{sw}\" height=\"{sw}\" rx=\"8\" fill=\"url(#rice-g-{e})\"/>\
        <rect x=\"2\" y=\"2\" width=\"{sw}\" height=\"{sw}\" rx=\"8\" fill=\"url(#rice-p-{e})\" opacity=\"1.0\"/>{dots}</svg>",
        s = s,
        sw = sw,
        d = defs(&eid1, rice, nori),
        e = eid1,
        dots = rice_ring_dots(s / 2.0, s / 2.0, s * 0.30, 14),
    );
    parts.push(format!(
        "<div class=\"sushi-exploded-part\" style=\"animation-delay:80ms\">{inner_rice}<span class=\"sushi-exploded-label\">Rice</span></div>\
        <span class=\"sushi-exploded-sep\" aria-hidden=\"true\">+</span>"
    ));
    for (i, &ing) in d.ingredients.iter().enumerate() {
        let m = ingredient_meta(ing);
        let eid = format!("{uid}I{i}");
        let tex = ingredient_texture_id(ing, &eid);
        let delay = 160 + i * 70;
        let ss = s * 0.8;
        let hx = s / 2.0;
        let hr = s * 0.44;
        let lx = hx - s * 0.1;
        let lr = s * 0.11;
        let inner_ing = format!(
            "<svg width=\"{ss}\" height=\"{ss}\" viewBox=\"0 0 {s} {s}\" role=\"img\" aria-label=\"{lb}\">{d}\
            <circle cx=\"{hx}\" cy=\"{hx}\" r=\"{hr:.1}\" fill=\"{f}\" stroke=\"{st}\" stroke-width=\"1.5\"/>\
            <circle cx=\"{hx}\" cy=\"{hx}\" r=\"{hr:.1}\" fill=\"{t}\" opacity=\"0.5\"/>\
            <circle cx=\"{lx:.1}\" cy=\"{lx:.1}\" r=\"{lr:.1}\" fill=\"rgba(255,255,255,0.2)\"/></svg>",
            ss = ss,
            s = s,
            lb = m.label,
            d = defs(&eid, rice, nori),
            hx = hx,
            hr = hr,
            f = m.fill,
            st = m.stroke,
            t = tex,
            lx = lx,
            lr = lr,
        );
        let sep = if i < d.ingredients.len() - 1 {
            "<span class=\"sushi-exploded-sep\" aria-hidden=\"true\">+</span>"
        } else {
            ""
        };
        parts.push(format!(
            "<div class=\"sushi-exploded-part\" style=\"animation-delay:{delay}ms\">{inner_ing}<span class=\"sushi-exploded-label\">{lb}</span></div>{sep}",
            delay = delay, lb = m.label
        ));
    }
    format!(
        "<div class=\"sushi-exploded-row\" role=\"img\" aria-label=\"{name} — exploded view\">{}</div>",
        parts.join(""),
        name = d.name
    )
}

/// Renders a complete sushi SVG or exploded HTML as a `String`.
///
/// The returned string is raw SVG / HTML safe to embed via any framework's
/// `inner_html` / `dangerous_inner_html` / `Html::from_html_unchecked` escape hatch.
///
/// # Arguments
/// * `data` - The [`SushiData`] describing shape, ingredients, size, and state.
/// * `scale` - Uniform scale factor applied to size and sheet thickness.
///
/// # Returns
/// A `String` of complete SVG or HTML markup.
pub fn render_sushi(data: &SushiData, scale: f64) -> String {
    let uid: String = data
        .id
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    let scaled = SushiData {
        size: crate::common::SushiSize {
            width: data.size.width * scale,
            height: data.size.height * scale,
        },
        outer_sheet: crate::common::SushiOuterSheet {
            color: data.outer_sheet.color.clone(),
            thickness: data.outer_sheet.thickness * scale,
        },
        ..data.clone()
    };
    if scaled.state == SushiState::Exploded {
        return exploded_view(&scaled, &uid);
    }
    match (scaled.view, scaled.shape) {
        (SushiView::Top, SushiShape::Circular) => circular_top(&scaled, &uid),
        (SushiView::Top, SushiShape::Square) => square_top(&scaled, &uid),
        (SushiView::Top, SushiShape::Triangular) => triangular_top(&scaled, &uid),
        (SushiView::Top, SushiShape::Oval) => oval_top(&scaled, &uid),
        (SushiView::Front, SushiShape::Circular) => circular_front(&scaled, &uid),
        (SushiView::Front, SushiShape::Square) => square_front(&scaled, &uid),
        (SushiView::Front, SushiShape::Triangular) => triangular_front(&scaled, &uid),
        (SushiView::Front, SushiShape::Oval) => oval_front(&scaled, &uid),
    }
}
