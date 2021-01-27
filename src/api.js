Deno.core.ops();

let Assets = {
    writeText : function() { System.log("Assets." + arguments.callee.name + "() not implemented"); },
    readText : function() { System.log("Assets." + arguments.callee.name + "() not implemented"); },
    getAssetList : function() { System.log("Assets." + arguments.callee.name + "() not implemented"); },
    setMaterialScroll : function() { System.log("Assets." + arguments.callee.name + "() not implemented"); },
    setMaterialWarp : function() { System.log("Assets." + arguments.callee.name + "() not implemented"); },
    setMaterialAnimation : function() { System.log("Assets." + arguments.callee.name + "() not implemented"); },
    setMaterialFullbright : function() { System.log("Assets." + arguments.callee.name + "() not implemented"); },
    exists : function() { System.log("Assets." + arguments.callee.name + "() not implemented"); },
}

let Audio = {
    play : function() { System.log("Audio." + arguments.callee.name + "() not implemented"); },
    playMusic : function() { System.log("Audio." + arguments.callee.name + "() not implemented"); },
    stopMusic : function() { System.log("Audio." + arguments.callee.name + "() not implemented"); },
}

let Config = {
    getInt : function() { System.log("Config." + arguments.callee.name + "() not implemented"); },
};

let Draw = {
    ref : null,
    rect : function() { 
        System.log("Draw." + arguments.callee.name + "() not implemented"); 
    },
    clear : function() { 
        Deno.core.jsonOpSync("draw_clear", { draw_ref: Draw.ref });  
    },
    text : function() { System.log("Draw." + arguments.callee.name + "() not implemented"); },
    line : function() { System.log("Draw." + arguments.callee.name + "() not implemented"); },
    line3d : function() { System.log("Draw." + arguments.callee.name + "() not implemented"); },
    cube : function() { System.log("Draw." + arguments.callee.name + "() not implemented"); },
    texture : function() { System.log("Draw." + arguments.callee.name + "() not implemented"); },

    texturePart : async function(x, y, textureName, xStart, yStart, width, height) {
        await Deno.core.jsonOpAsync("draw_texture_part", { 
            draw_ref: Draw.ref,
            x: x,
            y: y,
            textureName: textureName,
            xStart: xStart,
            yStart: yStart,
            width: width,
            height: height
        });  
    },

    mesh : function() { System.log("Draw." + arguments.callee.name + "() not implemented"); },
    textureScaled : function() { System.log("Draw." + arguments.callee.name + "() not implemented"); },
    textureTiled : function() { System.log("Draw." + arguments.callee.name + "() not implemented"); },
    getSize : function() { System.log("Draw." + arguments.callee.name + "() not implemented"); },
}

let Mesh = {
    load : function() { System.log("Mesh." + arguments.callee.name + "() not implemented"); },
    create : function() { System.log("Mesh." + arguments.callee.name + "() not implemented"); },
    generate : function() { System.log("Mesh." + arguments.callee.name + "() not implemented"); },
    removeAll : function() { System.log("Mesh." + arguments.callee.name + "() not implemented"); },
    combineColliders : function() { System.log("Mesh." + arguments.callee.name + "() not implemented"); },
    clone : function() { System.log("Mesh." + arguments.callee.name + "() not implemented"); },
    raycast : function() { System.log("Mesh." + arguments.callee.name + "() not implemented"); },
}

let System = {
    loadGame : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    loadMod : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    unloadMod : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    setResolution : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    quit : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    lockMouse : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    unlockMouse : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    hideMouse : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    showMouse : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    screenshot : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    reset : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    resetAssets : function() { System.log("System." + arguments.callee.name + "() not implemented"); },
    fogColor : function() { System.log("System." + arguments.callee.name + "() not implemented"); },

    log : function(message) { 
        Deno.core.jsonOpSync("log", message); 
    },
}

let camera = {
    x : 0,
    y : 0,
    z : 0,
    rotX : 0,
    rotY : 0,
    rotZ : 0,
    fov : 0,
    ortho : false,
    orthoScale : 0,
}
