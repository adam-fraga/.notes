/* Unity supports time-related operations through its Time library.
Time.time,Time.deltaTime and Time.timeScale are the most common APIs 
to work with time in your project. */

Time.time returns the time at the beginning of the current frame.

Time.deltaTime returns the time difference between the current and last frame in seconds.
(the time for each frame)

Time.timeScale represents scale at which time elapses.

Time.time returns the time at the beginning of the current frame.

Time.deltaTime returns the time difference between the current and last frame in seconds.

Time.timeScale represents scale at which time elapses.

// The time in seconds since the start of the game
float timeSinceStartOfGame = Time.time;

// The scale at which the time is passing
float currentTimeScale = Time.timeScale;

// Pause time
Time.timeScale = 0;

// The time in seconds it took to complete the last frame 
// Use with Update() and LateUpdate()
float timePassedSinceLastFrame = Time.deltaTime;

// The interval in seconds at which physics and fixed frame rate 
// updates are performed and use with FixedUpdate()
float physicsInterval =  Time.fixedDeltaTime;
