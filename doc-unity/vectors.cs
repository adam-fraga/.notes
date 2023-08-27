// We use vector to move entities in our 3 Space.
//X = Left/Right Y = Up/Down Z = Forward/Back

axis = (x, z, y)

Vector3.right /* (1, 0, 0) */   
Vector2.right /* (1, 0) */
Vector3.left /* (-1, 0, 0) */   
Vector2.left /* (-1, 0) */
Vector3.up /* (0, 1, 0) */      
Vector2.up /* (0, 1) */
Vector3.down /* (0, -1, 0) */   
Vector2.down /* (0, -1) */
Vector3.forward /* (0, 0, 1) */
Vector3.back /* (0, 0, -1) */
Vector3.zero /* (0, 0, 0) */    
Vector2.zero /* (0, 0) */
Vector3.one /* (1, 1, 1) */     
Vector2.one /* (1, 1) */

float length = myVector.magnitude /* Length of this Vector */

myVector.normalized /* Keeps direction, but reduces length to 1 */
