// struct Point {} // Private
// struct Matrix {} // Private

// struct Path {isHole, points}

// enum CompositeOperation {
//    SourceOver
//    SourceIn
//    SourceOut
//    Atop              // SourceAtop?
//    DestinationOver
//    DestinationIn
//    DestinationOut
//    DestinationAtop
//    Lighter
//    Copy
//    Xor
// }

// trait FillStyle {CompositeOperation}
// struct LinearGradient {}
// struct RadialGradient {}
// struct BoxGradient {}     // Needed?
// struct Image {}
// struct SolidColor {}


// // Consists of one or more paths, a transform, a fill, and a translation
// // Some paths may describe a "hole"
// struct Figure {
//    paths: Vec<Path>
//    transform: Matrix
//    fillStyles: Vec<FillStyle>
// }

// impl Figure {
//    fn new(Paths, Transform, FillStyles);
// }
// impl FigureBuilder {
//    // Translation
//    translate()
//    rotate()
//    scale()
//    skew()
//    matrix()

//    // Path describing
//    line_to()
//    bezier_to()
//    quadratic_to()
//    arc_to()
//    is_hole()
//    move_to() // Start new path
//    close()   // Start new path


//    // Fill
//    fill(FillStyle)     // Ends the builder, producing a Figure
//    fill_and(FillStyle) // Doesn't end the builder
// }


// struct Canvas {figures: Vec<Figure>}
// impl Canvas {
//    draw(windowWidth, windowHeight, pixelRatio)
// }

// // A Path is a collection of points describing a possibly closed shape
// // A Figure is a collection of paths, some of which may subtract from the others, along with a translation, fill
// // A Canvas is a collection of Figures describing an entire scene
// // Calling draw will execute openGL calls. To improve performance
// // collect all figures into a single canvas and pass that to draw instead
// // A canvas also lets you control anti-aliasing, highdpi, and other options?


// // Helper functions - return figures - I'm leaning against having these in the main library. Will need to test ergonomics
//    // Path creating (Last point should probably be undefined after this. Who knows where it could be? But people will still use it. So may as well make it defined. It's not like it really matters)
//    add_path()
//    arc()
//    rect()
//    rounded_rect()
//    circle()
//    ellipse()



// // I don't know that I want this to be part of the Figure builder
//    // Path Modifying
//    stroke(strokeWidth, miter, bevel) // Actually modifies all paths, duplicating them and scaling them to produce the correct stroke style


// const TransformIdentity







// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
