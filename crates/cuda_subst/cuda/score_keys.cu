#include <cuda_runtime.h>
#include <stdint.h>

extern "C" {

__constant__ float english_freq[26] = {
    8.2,1.5,2.8,4.2,12.7,2.2,2.0,6.1,7.0,0.2,
    0.8,4.0,2.4,6.7,7.5,1.9,0.1,6.0,6.3,9.1,
    2.8,1.0,2.4,0.2,2.0,0.1
};

__global__
void score_kernel(const char* ciphertext, int text_len,
                  const char* keys, int num_keys,
                  float* scores)
{
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx >= num_keys) return;

    const char* key = keys + idx * 26;

    float freq_score[26] = {0};

    for (int i = 0; i < text_len; i++) {
        char c = ciphertext[i];
        if (c >= 'A' && c <= 'Z') {
            char decoded = key[c - 'A'];
            freq_score[decoded - 'A'] += 1.0f;
        }
    }

    float sum = 0;
    for (int i = 0; i < 26; i++) sum += freq_score[i];
    if (sum == 0) { scores[idx] = -1e9; return; }

    float s = 0;
    for (int i = 0; i < 26; i++) {
        float observed = freq_score[i] / sum * 100.0f;
        float diff = observed - english_freq[i];
        s -= diff * diff;
    }

    scores[idx] = s;
}

extern "C"
void launch_score_kernel(const char* ciphertext, int text_len,
                         const char* keys, int num_keys,
                         float* scores)
{
    char *d_text, *d_keys;
    float *d_scores;

    cudaMalloc(&d_text, text_len);
    cudaMalloc(&d_keys, num_keys * 26);
    cudaMalloc(&d_scores, num_keys * sizeof(float));

    cudaMemcpy(d_text, ciphertext, text_len, cudaMemcpyHostToDevice);
    cudaMemcpy(d_keys, keys, num_keys * 26, cudaMemcpyHostToDevice);

    int threads = 4608;
    int blocks = (num_keys + threads - 1) / threads;

    score_kernel<<<blocks, threads>>>(d_text, text_len, d_keys, num_keys, d_scores);

    cudaMemcpy(scores, d_scores, num_keys * sizeof(float), cudaMemcpyDeviceToHost);

    cudaFree(d_text);
    cudaFree(d_keys);
    cudaFree(d_scores);
}

} // extern "C"
