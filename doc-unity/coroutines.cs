/* 
   A coroutine is like a background activity which can hold the execution 
   of codes after the yield statement until it returns a value.
*/

// Create a Coroutine 
private IEnumerator CountSeconds(int count = 10)
{
  for (int i = 0; i <= count; i++) {
    Debug.Log(i + " second(s) have passed");
    yield return new WaitForSeconds(1.0f);
  }
}

// Call a Coroutine 
StartCoroutine(CountSeconds());
StartCoroutine(CountSeconds(10));

// Store and call a Coroutine from a variable 
private IEnumerator countSecondsCoroutine;
countSecondsCoroutine = CountSeconds();
StartCoroutine(countSecondsCoroutine);

// Stop a stored Coroutine 
StopCoroutine(countSecondsCoroutine);

// Coroutine Return Types 
// Waits until the next Update() call
yield return null; 

// Waits until the next FixedUpdate() call
yield return new WaitForFixedUpdate(); 

// Waits until everything this frame has executed
yield return new WaitForEndOfFrame(); 

// Waits for game time in seconds
yield return new WaitForSeconds(float seconds); 

// Waits until a custom condition is met
yield return new WaitUntil(() => MY_CONDITION); 

// Waits for a web request
yield return new WWW("MY/WEB/REQUEST"); 

// Waits until another Coroutine is completed
yield return StartCoroutine("MY_COROUTINE");
