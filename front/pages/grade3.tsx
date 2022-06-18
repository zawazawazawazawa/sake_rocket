import React from "react";
import useSWR from 'swr';

type Distilleriy = {
  id: number,
  whisky_type: string,
  region: string | null,
  name: string,
  name_ja: string,
  owner: string,
  owner_ja: string
}

type Distilleries = Array<Distilleriy>

type Props = {
  props: Distilleries
}

const fetcher = async (url: string): Promise<Distilleries> => {
  return fetch(url)
    .then(response => {
      if (!response.ok) {
        throw new Error(response.statusText)
      }
      return response.json()
    })
}

export async function getServerSideProps(): Promise<Props> {
  const data: Distilleries = await fetcher("http://127.0.0.1:8000/distilleries");

  return {
    props: {
      ...data
    },
  };
}
  
const Grade3 = (props: Props) => {
  return (
    <>
      <div>Grade3</div>
    </>
  )
}
  
export default Grade3


