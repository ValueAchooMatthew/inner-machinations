export const load = ({cookies}) => {

    const email = cookies.get("email");

    return{
        email
    };

} 