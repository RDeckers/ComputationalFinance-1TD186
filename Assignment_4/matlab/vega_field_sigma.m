function [] = vega_field_sigma( N, M,  sigma_min, sigma_max, delta_sigma)
    V = compute_field_sigma(N, M,  sigma_min, sigma_max, delta_sigma);
    figure(2)
    mask = [-1 0 1]
    K=15; %strike price
    B = 2*K;
    T=0.5; %time of maturity
    deltaT = T/M;
    r=0.1; %rate of interest
    gamma = 1.0;

    S = generate_S(N, B, K);
    vega = diff(V.').'  ;
    size(vega)
    size(V)
    values = V(end-N+1:end);
    sigma= sigma_min+delta_sigma:delta_sigma:sigma_max;
    size(S)
    hold on;
    h= pcolor(sigma, S, vega)
    set(h,'edgecolor','none')
    hold off;
    hcb = colorbar('location','EastOutside');
    caxis([-0.3 0.1])
end

