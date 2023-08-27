// Create a GameObject
Instantiate(GameObject prefab);
Instantiate(GameObject prefab, Transform parent);
Instantiate(GameObject prefab, Vector3 position, Quaternion rotation);
Instantiate(bullet);
Instantiate(bullet, bulletSpawn.transform);
Instantiate(bullet, Vector3.zero, Quaternion.identity);
Instantiate(bullet, new Vector3(0, 0, 10), bullet.transform.rotation);

// Destroy a GameObject
Destroy(gameObject);

// Finding GameObjects
GameObject myObj = GameObject.Find("NAME IN HIERARCHY");
GameObject myObj = GameObject.FindWithTag("TAG");

// Accessing Components
Example myComponent = GetComponent<Example>();
AudioSource audioSource = GetComponent<AudioSource>();
Rigidbody rgbd = GetComponent<Rigidbody>();
