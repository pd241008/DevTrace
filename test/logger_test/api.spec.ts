import { test, expect } from '@playwright/test';

/**
 * DevTrace API Automation Suite
 * 
 * Target: DevTrace Rust Proxy (Default: http://127.0.0.1:8080)
 * Tech Stack: TypeScript + Playwright API Testing
 */

const BASE_URL = 'http://127.0.0.1:8080';

test.describe('DevTrace API Core - Functional Suite', () => {

    test('GET / - Should return system health and navigation', async ({ request }) => {
        const response = await request.get(`${BASE_URL}/`);
        expect(response.status()).toBe(200);
        const body = await response.json();
        expect(body.engine).toContain('DevTrace');
        expect(body.status).toBe('Active & Healthy');
    });

    test('GET /hello - Should trigger a log capture and return success', async ({ request }) => {
        const response = await request.get(`${BASE_URL}/hello`);
        expect(response.status()).toBe(200);
        const body = await response.json();
        expect(body.message).toContain('Hello from the DevTrace');
    });

    test('Integration: Intercepted request should appear in the logs', async ({ request }) => {
        const uniquePath = `/test-${Date.now()}`;
        await request.get(`${BASE_URL}${uniquePath}`);
        await new Promise(resolve => setTimeout(resolve, 500));

        const logRes = await request.get(`${BASE_URL}/logs/latest`);
        const latestLog = await logRes.json();
        expect(latestLog.request.path).toBe(uniquePath);
    });

    test('Negative: GET /invalid-route should return 404', async ({ request }) => {
        const response = await request.get(`${BASE_URL}/this-path-does-not-exist`);
        expect(response.status()).toBe(404);
    });

    test('Edge Case: Invalid status filter should return 400 Bad Request', async ({ request }) => {
        const response = await request.get(`${BASE_URL}/logs?status=not-a-number`);
        expect(response.status()).toBe(400);
        const body = await response.json();
        expect(body.error).toBeDefined();
    });
});
