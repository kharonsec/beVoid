import { createPromiseClient } from '@connectrpc/connect';
import { createGrpcWebTransport } from '@connectrpc/connect-web';
import { VoidService } from './gen/bevoid_pb';

const baseUrl = import.meta.env.VITE_BEVOID_URL ?? 'http://localhost:50051';

export const voidClient = createPromiseClient(VoidService, createGrpcWebTransport({ baseUrl }));
