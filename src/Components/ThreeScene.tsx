import { invoke } from '@tauri-apps/api/core';
import { EventCallback, listen, UnlistenFn } from '@tauri-apps/api/event';
import { useCallback, useEffect, useRef, useState } from 'react';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import { EffectComposer } from 'three/examples/jsm/postprocessing/EffectComposer.js';
import { RenderPass } from 'three/examples/jsm/postprocessing/RenderPass.js';
import { UnrealBloomPass } from 'three/examples/jsm/postprocessing/UnrealBloomPass.js';

interface StepData {
    row: number;
    col: number;
    value: number;
    grid: number[][];
}

export function ThreeScene() {
    const mountRef = useRef<HTMLDivElement>(null);
    const debugRef = useRef<HTMLDivElement>(null);
    const stepObjectsRef = useRef<THREE.Object3D[]>([]);
    const sphereMapRef = useRef<Map<string, THREE.Mesh>>(new Map());
    const solidStackRef = useRef<
        Array<{ key: string; sphere: THREE.Mesh; line: THREE.Line | null }>
    >([]);
    const transparentStackRef = useRef<
        Array<{ key: string; sphere: THREE.Mesh }>
    >([]);
    const lastStepRef = useRef<THREE.Vector3 | null>(null);

    const lastTimeRef = useRef(0);
    const stepQueueRef = useRef<StepData[]>([]);
    const keysRef = useRef({
        w: false,
        a: false,
        s: false,
        d: false,
        shift: false,
        space: false,
    });
    const startedRef = useRef(false);
    const finishedRef = useRef(false);

    const BLOOM_LAYER = 1;

    const [paused, setPaused] = useState(true);
    const [fastForward, setFastForward] = useState(false);

    const pausedRef = useRef(false);
    const fastRef = useRef(false);

    const [spacePressed, setSpacePressed] = useState(false);

    const debugLog = useCallback((msg: string) => {
        if (!keysRef.current.space) {
            return;
        }
        const box = debugRef.current;
        if (box) {
            box.textContent += msg + '\n';
            box.scrollTop = box.scrollHeight;
        }
    }, []);

    // Load initial Sudoku
    useEffect(() => {
        async function fetchSudoku() {
            const data = await invoke<number[][]>('get_sudoku');
            debugLog('Fetched sudoku grid');
            debugLog(JSON.stringify(data));
        }
        fetchSudoku();
    }, [debugLog]);

    // Listen for solver steps
    useEffect(() => {
        debugLog('🔌 Listening for sudoku-step events…');

        let unlisten: UnlistenFn | undefined;

        const handler: EventCallback<StepData> = (event: {
            payload: StepData;
        }) => {
            const step = event.payload;
            stepQueueRef.current.push(step);

            debugLog(
                `Step received → row=${step.row} col=${step.col} value=${step.value}`,
            );
        };

        listen<StepData>('sudoku-step', handler).then((fn) => {
            unlisten = fn;
        });

        return () => {
            debugLog('Unsubscribing sudoku-step listener');
            if (unlisten) {
                unlisten();
            }
        };
    }, [debugLog]);

    useEffect(() => {
        pausedRef.current = paused;
    }, [paused]);

    useEffect(() => {
        fastRef.current = fastForward;
    }, [fastForward]);

    const solveSudoku = useCallback(() => {
        debugLog('Requesting sudoku solve…');

        invoke<{
            steps: StepData[];
            grid: number[][];
            solved: boolean;
            unsolvable: boolean;
        }>('solve')
            .then(
                (result: {
                    steps: StepData[];
                    grid: { toString: () => string };
                    solved: boolean;
                    unsolvable: boolean;
                }) => {
                    debugLog(
                        `Solve returned with ${result.steps.length} steps`,
                    );

                    stepQueueRef.current = [...result.steps];

                    if (result.grid) {
                        debugLog(result.grid.toString());
                    }

                    if (result.solved) {
                        debugLog('Sudoku solved!');
                    }

                    if (result.unsolvable) {
                        debugLog('Sudoku is unsolvable.');
                    }
                },
            )
            .catch((err) => {
                console.error('Solve failed:', err);
            });
    }, [debugLog]);

    const applyStep = useCallback((step: StepData) => {
        invoke('update_cell', {
            row: step.row,
            col: step.col,
            val: step.value,
        });
    }, []);

    useEffect(() => {
        const sphereMapAtMount = sphereMapRef.current;
        const mount = mountRef.current;
        if (!mount) {
            return;
        }

        let animationId: number;

        const width = mount.clientWidth;
        const height = mount.clientHeight;

        const scene = new THREE.Scene();
        scene.background = new THREE.Color(0x222222);

        const camera = new THREE.PerspectiveCamera(
            75,
            width / height,
            0.1,
            1000,
        );
        camera.position.set(0, 0, 30);

        const light = new THREE.PointLight(0xffffff, 2, 100);
        light.position.set(10, 10, 10);
        scene.add(light);

        const axes = new THREE.AxesHelper(20);
        axes.position.set(-10, -8, -10);
        scene.add(axes);

        const renderer = new THREE.WebGLRenderer({ antialias: true });
        renderer.setSize(width, height);

        // ------- BLOOM EFFECT -------
        const composer = new EffectComposer(renderer);
        composer.addPass(new RenderPass(scene, camera));

        const bloomPass = new UnrealBloomPass(
            new THREE.Vector2(width, height),
            0.5, // strength (increase for more glow)
            0.4, // radius
            0.0, // threshold
        );
        bloomPass.renderToScreen = false;
        camera.layers.enable(BLOOM_LAYER);
        composer.addPass(bloomPass);

        mount.appendChild(renderer.domElement);

        const controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;

        // Movement keys

        const handleKeyDown = (event: KeyboardEvent) => {
            const key = event.code.toLowerCase();
            if (key in keysRef.current) {
                keysRef.current[key as keyof typeof keysRef.current] = true;
                if (key === 'space') {
                    setSpacePressed(true);
                }
            }
            if (key === 'e') {
                solveSudoku();
            }
        };

        const handleKeyUp = (event: KeyboardEvent) => {
            const key = event.code.toLowerCase();
            if (key in keysRef.current) {
                keysRef.current[key as keyof typeof keysRef.current] = false;
                if (key === 'space') {
                    setSpacePressed(false);
                }
            }
        };

        window.addEventListener('keydown', handleKeyDown);
        window.addEventListener('keyup', handleKeyUp);

        // Movement logic
        const baseSpeed = 0.05;
        const fastSpeed = 0.15;

        function updateMovement() {
            const speed = keysRef.current.shift ? fastSpeed : baseSpeed;

            const forward = new THREE.Vector3();
            camera.getWorldDirection(forward);

            const right = new THREE.Vector3();
            right.crossVectors(forward, camera.up);

            if (keysRef.current.w) {
                camera.position.addScaledVector(forward, speed);
                controls.target.addScaledVector(forward, speed);
            }
            if (keysRef.current.s) {
                camera.position.addScaledVector(forward, -speed);
                controls.target.addScaledVector(forward, -speed);
            }
            if (keysRef.current.a) {
                camera.position.addScaledVector(right, -speed);
                controls.target.addScaledVector(right, -speed);
            }
            if (keysRef.current.d) {
                camera.position.addScaledVector(right, speed);
                controls.target.addScaledVector(right, speed);
            }
        }

        function retractLine(line: THREE.Line, duration = 1000) {
            const geom = line.geometry as THREE.BufferGeometry;
            const count = geom.getAttribute('position').count;

            let progress = 1.0;
            const start = performance.now();

            function anim() {
                const t = (performance.now() - start) / duration;
                progress = 1 - t;

                if (progress <= 0) {
                    scene.remove(line);
                    geom.dispose();
                    if (Array.isArray(line.material)) {
                        line.material.forEach((m) => m.dispose());
                    }
                    else {
                        line.material.dispose();
                    }
                    return;
                }

                geom.setDrawRange(0, Math.floor(count * progress));
                requestAnimationFrame(anim);
            }
            anim();
        }

        function updateStep(scene: THREE.Scene) {
            if (pausedRef.current) {
                return;
            }

            const delay = fastRef.current ? 1 : 500;

            const now = performance.now();

            if (now - lastTimeRef.current < delay) {
                return;
            }
            lastTimeRef.current = now;

            const step = stepQueueRef.current.shift();
            if (!step) {
                if (!finishedRef.current && startedRef.current) {
                    invoke('force_update');
                    finishedRef.current = true;
                    startedRef.current = false;
                }
                return;
            }

            startedRef.current = true;

            const key = `${step.row},${step.col}`;
            const isBacktrack = step.value === 0;

            applyStep(step);

            if (isBacktrack) {
                debugLog(`Backtrack r=${step.row} c=${step.col}`);

                const last = solidStackRef.current.pop();
                if (last) {
                    // remove & dispose the original solid sphere
                    scene.remove(last.sphere);
                    if (last.sphere.geometry) {
                        last.sphere.geometry.dispose();
                    }
                    if (Array.isArray(last.sphere.material)) {
                        last.sphere.material.forEach((m) => m.dispose());
                    }
                    else {
                        last.sphere.material.dispose();
                    }
                    // remove & dispose line (if present)
                    if (last.line) {
                        retractLine(last.line); // <<--- animate instead of delete
                    }

                    // create a transparent (ghost) sphere at same position and save to transparentStack
                    const ghostMat = new THREE.MeshBasicMaterial({
                        color: 0xff0000,
                        transparent: true,
                        opacity: 0.2,
                    });
                    const ghost = new THREE.Mesh(
                        new THREE.SphereGeometry(0.2, 16, 16),
                        ghostMat,
                    );
                    ghost.position.copy(last.sphere.position);
                    scene.add(ghost);

                    // store ghost so we can clean it later
                    transparentStackRef.current.push({ key, sphere: ghost });

                    // update sphereMap to point to ghost (so future forward steps can reuse and overwrite)
                    sphereMapRef.current.set(key, ghost);
                }

                // Update lastStepRef to the most recent solid (non-backtracked) sphere position
                const newTop =
                    solidStackRef.current[solidStackRef.current.length - 1];
                lastStepRef.current = newTop
                    ? newTop.sphere.position.clone()
                    : null;
                return;
            }
            const pos = new THREE.Vector3(
                (4 - step.row) * 2,
                (step.col - 4) * 2,
                (step.value - 4) * 2,
            );

            const sphere = new THREE.Mesh(
                new THREE.SphereGeometry(0.2, 16, 16),
                new THREE.MeshStandardMaterial({
                    color: 0xffffff,
                    emissive: 0xffffff,
                    emissiveIntensity: 2.5,
                }),
            );

            scene.add(sphere);

            // using key+"#solid" prevents overwriting transparent ones
            const solidKey = key + '#solid';
            sphereMapRef.current.set(solidKey, sphere);

            // Make sphere opaque / solid
            const sphereMat = sphere.material as THREE.MeshStandardMaterial;
            sphereMat.transparent = false;
            sphereMat.opacity = 1;
            sphereMat.color.set(0xffffff);

            sphere.position.copy(pos);

            // Create line connecting last real step -> this pos
            let line: THREE.Line | null = null;
            if (lastStepRef.current) {
                const geometry = new THREE.BufferGeometry().setFromPoints([
                    lastStepRef.current.clone(),
                    pos.clone(),
                ]);
                const material = new THREE.LineBasicMaterial({
                    color: 0x00ff00,
                    transparent: false,
                });
                line = new THREE.Line(geometry, material);
                line.layers.set(0);
                scene.add(line);
            }

            // push onto solidStack
            solidStackRef.current.push({ key, sphere, line });

            // update map
            sphereMapRef.current.set(key, sphere);

            // update lastStepRef
            lastStepRef.current = pos.clone();

            debugLog(`Step r=${step.row} c=${step.col} v=${step.value}`);
        }

        function animate() {
            animationId = requestAnimationFrame(animate);
            updateMovement();
            updateStep(scene);
            controls.update();
            composer.render();
        }
        animate();

        const handleResize = () => {
            if (!mountRef.current) {
                return;
            }
            const w = mountRef.current.clientWidth;
            const h = mountRef.current.clientHeight;
            renderer.setSize(w, h);
            camera.aspect = w / h;
            camera.updateProjectionMatrix();
        };

        window.addEventListener('resize', handleResize);

        return () => {
            // ---- Stop animation ----
            cancelAnimationFrame(animationId);

            // ---- Remove global listeners ----
            window.removeEventListener('resize', handleResize);
            window.removeEventListener('keydown', handleKeyDown);
            window.removeEventListener('keyup', handleKeyUp);

            for (const entry of solidStackRef.current) {
                const { sphere, line } = entry;

                // solid sphere
                scene.remove(sphere);
                if (sphere.geometry) {
                    sphere.geometry.dispose();
                }
                if (Array.isArray(sphere.material)) {
                    sphere.material.forEach((m) => m.dispose());
                }
                else {
                    sphere.material.dispose();
                }

                // line
                if (line) {
                    scene.remove(line);
                    if (line.geometry) {
                        line.geometry.dispose();
                    }
                    if (Array.isArray(line.material)) {
                        line.material.forEach((m) => m.dispose());
                    }
                    else {
                        line.material.dispose();
                    }
                }
            }

            for (const entry of transparentStackRef.current) {
                const { sphere } = entry;

                scene.remove(sphere);
                if (sphere.geometry) {
                    sphere.geometry.dispose();
                }
                if (Array.isArray(sphere.material)) {
                    sphere.material.forEach((m) => m.dispose());
                }
                else {
                    sphere.material.dispose();
                }
            }

            for (const obj of stepObjectsRef.current) {
                scene.remove(obj);

                if (obj instanceof THREE.Mesh || obj instanceof THREE.Line) {
                    if (obj.geometry) {
                        obj.geometry.dispose();
                    }

                    if (Array.isArray(obj.material)) {
                        obj.material.forEach((m) => m.dispose());
                    }
                    else {
                        obj.material.dispose();
                    }
                }
            }

            // ---- Clear references ----
            solidStackRef.current = [];
            transparentStackRef.current = [];
            for (const obj of sphereMapAtMount.values()) {
                scene.remove(obj);
                if (obj.geometry) {
                    obj.geometry.dispose();
                }
                if (Array.isArray(obj.material)) {
                    obj.material.forEach((m) => m.dispose());
                }
                else {
                    obj.material.dispose();
                }
            }
            stepObjectsRef.current = [];
            lastStepRef.current = null;

            // ---- Dispose renderer / scene ----
            renderer.dispose();
            scene.clear();

            // ---- Remove canvas from DOM ----
            if (mount && renderer.domElement.parentNode === mount) {
                mount.removeChild(renderer.domElement);
            }
        };
    }, [applyStep, solveSudoku, debugLog]);

    return (
        <>
            <div
                style={{
                    position: 'relative',
                    width: '100%',
                    height: '100%',
                }}
            >
                <div
                    ref={mountRef}
                    style={{
                        width: '100%',
                        height: '100%',
                        position: 'absolute',
                        top: 0,
                        left: 0,
                    }}
                />

                <div
                    style={{
                        position: 'absolute',
                        top: 10,
                        left: 10,
                        display: 'flex',
                        gap: '10px',
                        zIndex: 10,
                        pointerEvents: 'auto',
                    }}
                >
                    <button
                        onClick={() => {
                            // if currently paused and there's no pending steps, start the solver
                            setPaused((prev) => {
                                const willPause = !prev;
                                if (prev && stepQueueRef.current.length === 0) {
                                    // first time pressing Play -> request solver (same as pressing 'e')
                                    solveSudoku();
                                }
                                return willPause;
                            });
                        }}
                        style={{ padding: '8px 14px' }}
                    >
                        {paused ? '▶ Play' : '⏸ Pause'}
                    </button>

                    <button
                        onClick={() => setFastForward((f) => !f)}
                        style={{ padding: '8px 14px' }}
                    >
                        {fastForward ? '⏪ Normal Speed' : '⏩ Fast Forward'}
                    </button>
                </div>

                <div
                    ref={debugRef}
                    style={
                        spacePressed
                            ? {
                                  display: 'block',
                                  position: 'absolute',
                                  bottom: '10px',
                                  left: '10px',
                                  padding: '6px 10px',
                                  background: 'rgba(0,0,0,0.7)',
                                  color: '#0f0',
                                  fontSize: '12px',
                                  maxWidth: '40%',
                                  maxHeight: '35%',
                                  overflowY: 'auto',
                                  pointerEvents: 'none',
                                  whiteSpace: 'pre-wrap',
                                  fontFamily: 'monospace',
                                  borderRadius: '6px',
                                  zIndex: 10,
                              }
                            : { display: 'none' }
                    }
                />
            </div>
        </>
    );
}
