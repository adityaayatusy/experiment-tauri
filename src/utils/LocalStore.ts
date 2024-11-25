import { load, Store } from "@tauri-apps/plugin-store";
import {appDataDir} from "@tauri-apps/api/path";
import {Client, Stronghold} from "@tauri-apps/plugin-stronghold";

class LocalStore {
    private static store: Store; // Static store

    private static async init() {
        if (!this.store) {
            this.store = await load('store.json', { autoSave: true });
        }

        return this.store;
    }

    public static async get<T>(key: string): Promise<T | undefined> {
        const store = await this.init();
        return store.get<T>(key);
    }

    public static async set<T>(key: string, value: T): Promise<void> {
        const store = await this.init();
        await store.set(key, value);
        await store.save();
    }

    public static async has(key: string) {

        console.log("Start");
        const vaultPath = `${await appDataDir()}/vault.hold`;
        console.log("vaultPath");
        const vaultPassword = 'secret';
        console.log("vaultPassword");
        const stronghold = await Stronghold.load(vaultPath, vaultPassword);

        console.log("vaultPassword");
        console.log("Vault Password:", vaultPassword);
        console.log("Stronghold instance:", stronghold);

        let client: Client;
        const clientName = 'test';

        try {
            console.log("Attempting to load client:", clientName);
            client = await stronghold.loadClient(clientName);
            console.log("Client loaded successfully");
        } catch (error) {
            console.log("Error loading client, attempting to create new client:", error);
            client = await stronghold.createClient(clientName);
            console.log("Client created successfully");
        }

        const store = client.getStore();
        console.log("Store instance:", store);

        const data = Array.from(new TextEncoder().encode(`test ${Date.now()}`));
        console.log("Data to insert:", data);

        await store.insert(key, data);
        console.log("Data inserted successfully");

        const dataGet: any = await store.get(key);
        console.log("Data retrieved:", dataGet);

        const value = new TextDecoder().decode(new Uint8Array(dataGet));
        console.log("Decoded value:", value);

        await stronghold.save();
        console.log("Stronghold saved successfully");
    }
}

export default LocalStore;
