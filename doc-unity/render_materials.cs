/* 
   Materials tell how a surface should be rendered in the scene.
   The material contains references to shaders, textures, color, emission 
   and more. Every material requires a shader for rendering the content 
   and the attributes available for that may vary on different shader values.
*/

[SerializeField] Material material;
[SerializeField] Texture2D texture;
[SerializeField] Color color = Color.red;

// Start is called before the first frame update
void Start() {
    MeshRenderer meshRenderer = GetComponent<MeshRenderer>();
    // Changing material, texture, color and shader at runtime
    meshRenderer.material = material;
    meshRenderer.material.mainTexture = texture;
    meshRenderer.material.color = color;
    meshRenderer.material.SetColor("_Color", Color.blue);
    meshRenderer.material.EnableKeyword("_EMISSION");
    meshRenderer.material.SetColor("_EmissionColor", Color.yellow);
    meshRenderer.material.shader = Shader.Find("Standard (Specular setup)");
}
