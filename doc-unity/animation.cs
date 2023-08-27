/*
  Unity made animation simple with the help of Animator controls and Animation Graph.
  Unity calls the animator controllers to handle which animations to play 
  and when to play them. The animation component is used to playback animations.

  It is quite simple to interact with the animation from a script.
  First, you have to refer the animation clips to the animation component. 
  Then get the Animator component reference in the script via GetComponent 
  method or by making that variable public. 
  Finally, set enabled attribute value to true for enabling the animation 
  and false for disabling it.
*/

[SerializeField] GameObject cube;
cube.GetComponent<Animator>().enabled = true;
cube.GetComponent<Animator>().SetTrigger("Enable");
cube.GetComponent<Animator>().ResetTrigger("Enable");
cube.GetComponent<Animator>().SetInteger("animId", 1);
