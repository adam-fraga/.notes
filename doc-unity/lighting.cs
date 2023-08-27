/*
   Lighting is a mandatory component in any Unity scene.
   All unity scenes contain a directional light component in it by default.
   Unity has four types of lights — directional, points, spot and area lights.
*/


[SerializeField] LightType lightType = LightType.Directional;
Light lightComp = null;

// Start is called before the first frame update
void Start() {
 GameObject lightGameObject = new GameObject("The Light");
 lightComp = lightGameObject.AddComponent<Light>();
 lightComp.color = Color.blue;
 lightComp.type = lightType;
 lightGameObject.transform.position = new Vector3(0, 5, 0);
}

void Update() {
 if (Input.GetKey(KeyCode.UpArrow)) 
    lightComp.GetComponent<Light>().enabled = true; 
 if (Input.GetKey(KeyCode.DownArrow)) 
    lightComp.GetComponent<Light>().enabled = false;
}
