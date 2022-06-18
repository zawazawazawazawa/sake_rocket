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
  props: {
    distilleries: Distilleries
  }
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
      distilleries: data
    },
  };
}
  
const Grade3 = (props: {distilleries: Distilleries}) => {
  const distilleries = props.distilleries.map((distillery) =>
    <>
      <li>id: {distillery.id}</li>
      <li>whisky type: {distillery.whisky_type}</li>
      <li>region: {distillery.region}</li>
      <li>name: {distillery.name}</li>
      <li>蒸留所名: {distillery.name_ja}</li>
      <li>owner: {distillery.owner}</li>
      <li>オーナー: {distillery.owner_ja}</li>
      <br />
    </>
  )

  return (
    <>
      <div>Grade3</div>

      <ul>{distilleries}</ul>
    </>
  )
}
  
export default Grade3


