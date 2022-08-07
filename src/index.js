const rust = import('../pkg');
const canvas = document.getElementById('canvas');
const gl = canvas.getContext('webgl', {antialias: true});

rust.then(m => {
    if (!gl) {
        alert('WebGL could not be initialized.');
        return;
    }

    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    const FPS = 30.0;
    const BUFFER = 1000.0 / FPS;
    const CLIENT = new m.Client();
    const initTime = Date.now();

    var prevTime = -1;

    function render() {
        window.requestAnimationFrame(render);
        
        const curTime = Date.now();
        if (curTime >= prevTime + BUFFER) {
            prevTime = curTime;
            console.log("HI");

            if (window.innerHeight != canvas.height ||
                window.innerWidth != canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;
                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;
                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            CLIENT.update(curTime - initTime,
                          window.innerHeight,
                          window.innerWidth);
            CLIENT.render();
        }
    }

    render();
});
