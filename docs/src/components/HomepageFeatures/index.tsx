import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: 'Parallel Execution',
    Svg: require('@site/static/img/undraw_not_found_re_bh2e.svg').default,
    description: (
      <>
        Speed up your test suite with parallel execution, 
        ideal for CI/CD pipelines.
      </>
    ),
  },
  {
    title: 'JSON Data Import/Export',
    Svg: require('@site/static/img/undraw_elements_re_25t9.svg').default,
    description: (
      <>
        Import test scenarios via JSON and export results in JSON format, 
        perfect for data-driven testing and CI/CD integration.
      </>
    ),
  },
  {
    title: 'OpenAPI Schema Support',
    Svg: require('@site/static/img/undraw_good_team_re_hrvm.svg').default,
    description: (
      <>
        Generate tests directly from your OpenAPI schema, 
        streamlining test creation and ensuring API specification compliance.
      </>
    ),
  },
];

function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
