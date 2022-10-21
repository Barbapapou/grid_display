varying highp vec2 vTextureCoord;

uniform vec3 uColor;
uniform sampler2D uSampler;

void main(void) {
    //    highp vec4 texelColor = texture2D(uSampler, vTextureCoord);
    //    gl_FragColor = vec4(texelColor.rgb * uColor, texelColor.a);
    gl_FragColor = vec4(uColor, 1.0);
}