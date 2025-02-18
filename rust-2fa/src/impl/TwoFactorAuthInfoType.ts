export type TwoFactorAuthInfoType = {
    company: string;
    custom_name: string;
    totp_code: number;
    username: string;
    remaining_time: number;
};