import type {Ref} from "vue";
import type {ApiError} from "~/composables/fetchApi";

export enum UserStatus {
    Unconfirmed = 'Unconfirmed',
    Normal = 'Normal',
    Banned = 'Banned',
    Admin = 'Admin',
    NotConnected = 'NotConnected',
    Unknown = 'Unknown'
}

export type AuthStatus = {
    status: UserStatus
    name: string
    email: string
}

export type SignInResponse = {
    status: UserStatus
    name: string
    id: string
    auth_token: string
}


export const useUserStore = defineStore('user', () => {

    // Data
    const status = ref(UserStatus.Unknown)
    const name: Ref<string | null> = ref(null)
    const email: Ref<string | null> = ref(null)
    let id = useCookie('pm_user_id', {watch: true})
    let auth_token = useCookie('pm_auth_token', {watch: true})

    // Methods
    const isConnected = (unconfirmed: boolean, banned: boolean) => {
        return status.value != UserStatus.NotConnected && status.value != UserStatus.Unknown
            && (banned || status.value != UserStatus.Unconfirmed)
            && (unconfirmed || status.value != UserStatus.Banned)
    }
    const isUnconfirmed = () => {
        return status.value == UserStatus.Unconfirmed
    }
    const isAdmin = () => {
        return status.value == UserStatus.Admin
    }
    const signIn = (email: string, password: string) => {
        return useFetchApi(false, 'POST', null, null, '/auth/signin', {email, password},
            (data: SignInResponse) => {
                status.value = data.status
                name.value = data.name
                id.value = data.id
                auth_token.value = data.auth_token
            }, (error: ApiError | null) => {
                if (error && error.error_type === ErrorType.Unauthorized) {
                    status.value = UserStatus.NotConnected
                } else {
                    status.value = UserStatus.Unknown
                }
            });
    }

    const fetchStatus = async () => {
        // id = useCookie('pm_user_id')
        // auth_token = useCookie('pm_auth_token')
        if (id.value && auth_token.value) {
            await useFetchApi(true, 'GET', auth_token.value, id.value, '/auth/status', null,
                (data: AuthStatus) => {
                    status.value = data.status
                    name.value = data.name
                    email.value = data.email
                },
                (error: ApiError | null) => {
                    if (error && error.error_type === ErrorType.Unauthorized) {
                        status.value = UserStatus.NotConnected
                    } else {
                        status.value = UserStatus.Unknown
                    }
                    // id.value = null
                    // auth_token.value = null
                });
        } else {
            status.value = UserStatus.NotConnected
        }
    }


    return {
        status, name, email, id, auth_token,
        isConnected, isUnconfirmed, isAdmin, signIn,
        fetchStatus
    }
})
