using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class HelloWorld : MonoBehaviour
{
    // Make visible property game object in unity UI is not recommanded
    public GameObject gameObject1; 

    // Use private and SerializeField instead
    [SerializeField]
    private GameObject gameObject2;

    // Private made them accessible in the script only
    private GameObject gameObject3;

    // Start is called before the first frame update
    private void Start()
    {
        Debug.Log("Hello World");
    }    
    // Update is called once per frame
    private void Update() {}

    // Called when the script is being loaded
    private void Awake() {}

    // Called every time the object is enabled
    private void OnEnable() {}

    // Called on the frame when the script is enabled
    private void Start() {}

    // Called once per frame
    private void Update() {}

    // Called every frame after Update
    private void LateUpdate() {}

    // Called every Fixed Timestep
    private void FixedUpdate() {}

    // Called when the renderer is visible by any Camera
    private void OnBecameVisible() {}

    // Called when the renderer is no longer visible by any Camera
    private void OnBecameInvisible() {}

    // Allows you to draw Gizmos in the Scene View
    private void OnDrawGizmos() {}

    // Called multiple times per frame in response to GUI events
    private void OnGUI() {}

    // Called at the end of a frame when a pause is detected
    private void OnApplicationPause() {}

    // Called every time the object is disabled
    private void OnDisable() {}

    // Only called on previously active GameObjects that have been destroyed
    private void OnDestroy() {}
}
