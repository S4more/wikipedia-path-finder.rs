import BaseInput from "../components/inputs/base-input";
import ButtonBase from "../components/inputs/button";
import ButtonSubmit from "../components/inputs/button-submit";

export default function Example() {
    return <>
        <div className="flex min-h-full items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
            <div className="w-full max-w-md space-y-8">
                <div>
                    <h2 className="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900">
                        Find a path between pages
                    </h2>
                </div>
                <form className="mt-8 space-y-6" action="#" method="POST">
                    <BaseInput type="text" required placeholder="From" />
                    <BaseInput type="text" required placeholder="To" />
                    <div className="flex">
                        <ButtonBase>Back</ButtonBase>
                        <ButtonSubmit type="submit">Search</ButtonSubmit>
                    </div>
                </form>
            </div>
        </div>
    </>
}