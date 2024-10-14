import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
    vus: 200, // Número de usuários virtuais
    duration: '60s', // Duração do teste
};

export default function () {
    const res = http.get('http://localhost:8080/api/v1/client/67723050003');
    const res2 = http.get('http://localhost:8080/api/v1/product?category_id=2');
    
    check(res, {
        'client is status 200': (r) => r.status === 200,
    });

    check(res2, {
        'product is status 200': (r) => r.status === 200,
    });

    sleep(0.1);
}
