// Input system API to handle key pressed by the user

// Stay true for as long as the key is pressed (ideal for movement control)
if (Input.GetKeyDown(KeyCode.Space)) { 
    Debug.Log("Space key was Pressed"); 
}
// Return true for a single frame when the key is pressed (ideal for jump)
if (Input.GetKeyUp(KeyCode.W)) { 
    Debug.Log("W key was Released"); 
}

if (Input.GetKey(KeyCode.UpArrow)) { 
    Debug.Log("Up Arrow key is being held down"); 
}
/* Button Input located under Edit >> Project Settings >> Input */
if (Input.GetButtonDown("ButtonName")) { 
    Debug.Log("Button was pressed"); 
}

if (Input.GetButtonUp("ButtonName")) { 
    Debug.Log("Button was released"); 
}

if (Input.GetButton("ButtonName")) { 
    Debug.Log("Button is being held down"); 
}
