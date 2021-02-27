#type vertex
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;

uniform mat4 uProjection;
uniform mat4 uView;
uniform mat4 uTransformation;

out vec3 ourColor;

void main()
{
    gl_Position = uProjection /* * uView */ * uTransformation * vec4(aPos.x, aPos.y, aPos.z, 1.0);
    ourColor = aColor;
}

#type fragment
#version 330 core
out vec4 FragColor;

in vec3 ourColor;

void main()
{
    FragColor = vec4(ourColor, 1.0);
}
